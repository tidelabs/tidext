#[macro_use]
extern crate napi_derive;

use std::{convert::TryInto, path::PathBuf};

use anyhow::anyhow;
use napi::bindgen_prelude::{Buffer, Error, Result, Status};
use tidext::{
  primitives::AccountId,
  stronghold::{Location, ProcResult, Procedure, ResultMessage, Stronghold},
  Client as SubstrateClient, ClientBuilder as SubstrateClientBuilder, Permill, TidefiKeyring,
};
use zeroize::Zeroize;

mod wrapper;

const SECRET_VAULT_PATH: &str = "TIDEFI";
const SR25519_KEYPAIR_RECORD_PATH: &str = "SR25519_KEYPAIR";

fn err_mapper<T: std::fmt::Display>(e: T) -> Error {
  Error::new(Status::GenericFailure, format!("{}", e))
}

fn anyhow_err_mapper(e: anyhow::Error) -> Error {
  Error::new(Status::GenericFailure, format!("{:#}", e))
}

fn password_to_encryption_key(mut password: Vec<u8>) -> [u8; 32] {
  let mut dk = [0; 64];
  // safe to unwrap (rounds > 0)
  crypto::keys::pbkdf::PBKDF2_HMAC_SHA512(&password, b"TIDEFI", 256, &mut dk).unwrap();
  password.zeroize();
  let key: [u8; 32] = dk[0..32][..].try_into().unwrap();
  key
}

fn stronghold_response_to_result<T>(status: ResultMessage<T>) -> std::result::Result<T, String> {
  match status {
    ResultMessage::Ok(v) => Ok(v),
    ResultMessage::Error(e) => Err(e),
  }
}

async fn get_signer(
  stronghold: Stronghold,
  keypair_location: Location,
) -> anyhow::Result<TidefiKeyring> {
  let pk = match stronghold
    .runtime_exec(Procedure::Sr25519PublicKey {
      keypair: keypair_location.clone(),
    })
    .await
  {
    ProcResult::Sr25519PublicKey(ResultMessage::Ok(pk)) => pk,
    _ => return Err(anyhow!("Failed to read public key")),
  };
  Ok(TidefiKeyring::new(
    AccountId::from(pk.inner().0),
    stronghold,
    keypair_location,
  ))
}

#[napi(object)]
pub struct Currency {
  pub token_id: wrapper::CurrencyId,
  pub metadata: wrapper::CurrencyMetadata,
}

#[napi]
pub struct Builder {
  url: String,
  snapshot_path: String,
  password: String,
}

#[napi]
impl Builder {
  /// Initializes the Builder.
  #[napi(constructor)]
  pub fn new(url: String, snapshot_path: String, password: String) -> Self {
    Self {
      url,
      snapshot_path,
      password,
    }
  }

  #[napi]
  pub async fn build(&self) -> Result<Client> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
      let system = actix::System::new();
      let stronghold = system
        .block_on(Stronghold::init_stronghold_system(Vec::new(), Vec::new()))
        .unwrap();
      tx.send((stronghold, actix::System::current())).unwrap();
      system.run().expect("failed to run actix system");
    });
    let (mut stronghold, system) = rx.recv().unwrap();

    let snapshot_path = PathBuf::from(self.snapshot_path.clone());
    if snapshot_path.exists() {
      let res = stronghold
        .read_snapshot(
          Vec::new(),
          None,
          &password_to_encryption_key(self.password.as_bytes().to_vec()).to_vec(),
          None,
          Some(snapshot_path),
        )
        .await;
      if let Err(e) = stronghold_response_to_result(res) {
        system.stop();
        return Err(Error::new(Status::GenericFailure, e));
      }
    }

    let builder = SubstrateClientBuilder::new()
      .set_signer(
        get_signer(
          stronghold,
          Location::generic(SECRET_VAULT_PATH, SR25519_KEYPAIR_RECORD_PATH),
        )
        .await
        .map_err(anyhow_err_mapper)?
        .pair_signer(),
      )
      .set_url(&self.url);

    Ok(Client {
      inner: builder.build().await.map_err(err_mapper)?,
    })
  }
}

#[napi]
pub struct Client {
  inner: SubstrateClient,
}

#[napi]
impl Client {
  #[napi]
  pub async fn system_health(&self) -> Result<wrapper::NodeHealth> {
    self
      .inner
      .system_health()
      .await
      .map(Into::into)
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn get_account_id(&self) -> Buffer {
    let id = self.inner.account_id();
    Buffer::from(id.as_ref())
  }

  #[napi]
  pub async fn get_account_id_ss58(&self) -> String {
    self.inner.account_id().to_string()
  }

  #[napi]
  pub async fn get_regular_swap_fee(&self) -> Result<u32> {
    self
      .inner
      .swap_fee()
      .await
      .map(|a| a.deconstruct())
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn get_market_maker_swap_fee(&self) -> Result<u32> {
    self
      .inner
      .swap_fee_market_maker()
      .await
      .map(|a| a.deconstruct())
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn extrinsic_cost(&self, extrinsic: String) -> Result<wrapper::BalanceInfo> {
    let amount = self
      .inner
      .extrinsic_cost(extrinsic)
      .await
      .map_err(err_mapper)?;
    Ok(wrapper::balance_info_from(amount))
  }

  #[napi]
  pub async fn submit_signed_extrinsic(&self, extrinsic: String) -> Result<()> {
    self
      .inner
      .submit_signed_extrinsic(extrinsic)
      .await
      .map(|_| ())
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn unstake_extrinsic(&self, stake_id: Buffer, force_unstake: bool) -> Result<String> {
    let stake_id: [u8; 32] = stake_id[0..32][..]
      .try_into()
      .expect("invalid stake id value");
    self
      .inner
      .unstake_extrinsic(stake_id.into(), force_unstake)
      .await
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn stake_extrinsic(
    &self,
    token_id: wrapper::CurrencyId,
    amount: wrapper::BalanceInfo,
    duration: u32,
  ) -> Result<String> {
    let extrinsic = self
      .inner
      .stake_extrinsic(
        wrapper::currency_id_into(token_id),
        wrapper::balance_info_into(amount),
        duration,
      )
      .await
      .map_err(err_mapper)?;
    Ok(extrinsic)
  }

  #[napi]
  pub async fn swap_extrinsic(
    &self,
    from_token_id: wrapper::CurrencyId,
    to_token_id: wrapper::CurrencyId,
    from_amount: wrapper::BalanceInfo,
    to_amount: wrapper::BalanceInfo,
    swap_type: wrapper::SwapType,
    slippage_tolerance: u32,
  ) -> Result<String> {
    self
      .inner
      .swap_extrinsic(
        wrapper::currency_id_into(from_token_id),
        wrapper::balance_info_into(from_amount),
        wrapper::currency_id_into(to_token_id),
        wrapper::balance_info_into(to_amount),
        swap_type.into(),
        Some(Permill::from_rational(slippage_tolerance, 1_000_000)),
      )
      .await
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn transfer_extrinsic(
    &self,
    token_id: wrapper::CurrencyId,
    amount: wrapper::BalanceInfo,
    destination: Buffer,
  ) -> Result<String> {
    let destination: [u8; 32] = destination[0..32][..]
      .try_into()
      .expect("invalid destination value");
    self
      .inner
      .transfer_extrinsic(
        AccountId::from(destination),
        wrapper::currency_id_into(token_id),
        wrapper::balance_info_into(amount),
      )
      .await
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn balance(
    &self,
    token_id: wrapper::CurrencyId,
    account_id: Option<Buffer>,
  ) -> Result<wrapper::CurrencyBalance> {
    let account_id = if let Some(id) = account_id {
      let id: [u8; 32] = id[0..32][..].try_into().expect("invalid account id value");
      AccountId::from(id)
    } else {
      self.inner.account_id().clone()
    };

    let token_balance = self
      .inner
      .balance(&account_id, wrapper::currency_id_into(token_id))
      .await
      .map_err(err_mapper)?;
    let balance = wrapper::CurrencyBalance {
      available: wrapper::balance_info_from(token_balance.available),
      reserved: wrapper::balance_info_from(token_balance.reserved),
    };
    Ok(balance)
  }

  #[napi]
  pub async fn total_stake_for(
    &self,
    currency_id: wrapper::CurrencyId,
  ) -> Result<wrapper::BalanceInfo> {
    let amount = self
      .inner
      .total_stake_for(wrapper::currency_id_into(currency_id))
      .await
      .map_err(err_mapper)?;
    Ok(wrapper::balance_info_from(amount))
  }

  #[napi]
  pub async fn total_supply_for(
    &self,
    currency_id: wrapper::CurrencyId,
  ) -> Result<wrapper::BalanceInfo> {
    let amount = self
      .inner
      .total_supply_for(wrapper::currency_id_into(currency_id))
      .await
      .map_err(err_mapper)?;
    Ok(wrapper::balance_info_from(amount))
  }

  #[napi]
  pub async fn all_assets(&self) -> Result<Vec<Currency>> {
    self
      .inner
      .all_assets()
      .await
      .map(|assets| {
        assets
          .into_iter()
          .map(|(id, metadata)| Currency {
            token_id: wrapper::currency_id_from(id),
            metadata: metadata.into(),
          })
          .collect()
      })
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn withdrawal(
    &self,
    asset_id: wrapper::CurrencyId,
    amount: wrapper::BalanceInfo,
    external_address: Buffer,
  ) -> Result<()> {
    self
      .inner
      .withdrawal(
        wrapper::currency_id_into(asset_id),
        wrapper::balance_info_into(amount),
        external_address.into(),
      )
      .await
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn withdrawal_extrinsic(
    &self,
    asset_id: wrapper::CurrencyId,
    amount: wrapper::BalanceInfo,
    external_address: Buffer,
  ) -> Result<String> {
    self
      .inner
      .withdrawal_extrinsic(
        wrapper::currency_id_into(asset_id),
        wrapper::balance_info_into(amount),
        external_address.into(),
      )
      .await
      .map_err(err_mapper)
  }
}
