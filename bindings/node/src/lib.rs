// Copyright 2021-2022 Semantic Network Ltd.
// This file is part of tidext.

// tidext is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// tidext is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with tidext.  If not, see <http://www.gnu.org/licenses/>.

#[macro_use]
extern crate napi_derive;

use std::{convert::TryInto, path::PathBuf};

use napi::bindgen_prelude::{Buffer, Error, Result, Status};
use tidext::{
  init_stronghold_from_seed,
  primitives::AccountId,
  stronghold::{KeyProvider, Location, SnapshotPath},
  Client as SubstrateClient, ClientBuilder as SubstrateClientBuilder, Permill, TidechainCall,
  TidefiCall, TidefiKeyring,
};
use zeroize::Zeroize;

mod wrapper;

const SECRET_VAULT_PATH: &str = "TIDEFI";
const SR25519_KEYPAIR_RECORD_PATH: &str = "SR25519_KEYPAIR";
const SECRET_KEY_FILE_EXTENSION: &str = "secret-key";
const SECRET_KEY_LENGTH: usize = 128;

fn err_mapper<T: std::fmt::Display>(e: T) -> Error {
  Error::new(Status::GenericFailure, format!("{}", e))
}

fn hash_password(mut password: Vec<u8>, mut secret_key: Vec<u8>) -> Vec<u8> {
  let config = argon2::Config {
    lanes: 2,
    mem_cost: 50_000,
    time_cost: 30,
    thread_mode: argon2::ThreadMode::from_threads(2),
    variant: argon2::Variant::Argon2id,
    ..Default::default()
  };

  let key = argon2::hash_raw(password.as_ref(), secret_key.as_ref(), &config)
    .expect("failed to hash password");

  password.zeroize();
  secret_key.zeroize();

  key
}

#[napi(object)]
pub struct Currency {
  pub token_id: wrapper::CurrencyId,
  pub metadata: wrapper::CurrencyMetadata,
}

#[napi]
pub struct Builder {
  url: String,
  client_path: Vec<u8>,
  snapshot_path: String,
  password: String,
}

#[napi]
impl Builder {
  /// Initializes the Builder.
  #[napi(constructor)]
  pub fn new(url: String, client_path: Vec<u8>, snapshot_path: String, password: String) -> Self {
    Self {
      client_path,
      url,
      snapshot_path,
      password,
    }
  }

  #[napi]
  pub async fn build(&self) -> Result<Client> {
    let stronghold_path: PathBuf = self.snapshot_path.clone().into();
    let location = Location::generic(SECRET_VAULT_PATH, SR25519_KEYPAIR_RECORD_PATH);

    try_build(
      &self.url,
      self.client_path.clone(),
      stronghold_path,
      location,
      &self.password,
    )
    .await
  }
}

async fn try_build(
  url: &str,
  client_path: Vec<u8>,
  stronghold_path: PathBuf,
  location: Location,
  password: &str,
) -> Result<Client> {
  let secret_key_path = stronghold_path
    .as_path()
    .with_extension(SECRET_KEY_FILE_EXTENSION);

  let builder = SubstrateClientBuilder::new()
    .set_signer(if stronghold_path.exists() {
      let secret_key = std::fs::read(secret_key_path).map_err(err_mapper)?;
      TidefiKeyring::try_from_stronghold_path(
        client_path.clone(),
        &stronghold_path,
        Some(location),
        Some(hash_password(password.as_bytes().to_vec(), secret_key)),
      )
      .map_err(err_mapper)?
    } else {
      let mut secret_key = [0u8; SECRET_KEY_LENGTH];
      crypto::utils::rand::fill(&mut secret_key).map_err(err_mapper)?;

      let stronghold = init_stronghold_from_seed(client_path.clone(), &location, None, None)
        .map_err(err_mapper)?;

      let snapshot_path = SnapshotPath::named(stronghold_path);
      let key_provider = KeyProvider::try_from(hash_password(
        password.as_bytes().to_vec(),
        secret_key.to_vec(),
      ))
      .map_err(err_mapper)?;

      stronghold
        .commit_with_keyprovider(&snapshot_path, &key_provider)
        .map_err(err_mapper)?;
      std::fs::write(secret_key_path, &secret_key)?;

      TidefiKeyring::try_from_stronghold_instance(client_path, stronghold, Some(location))
        .map_err(err_mapper)?
    })
    .set_url(url);

  Ok(Client {
    inner: builder.build().await.map_err(err_mapper)?,
  })
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
    let id = self.inner.account_id().expect("No signer found");
    Buffer::from(id.as_ref())
  }

  #[napi]
  pub async fn get_account_id_ss58(&self) -> String {
    self.inner.account_id_ss58().expect("No signer found")
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
  pub async fn submit_signed_extrinsic(&self, extrinsic: String) -> Result<String> {
    self
      .inner
      .submit_signed_extrinsic(extrinsic)
      .await
      .map(wrapper::hash_to_string)
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
  pub async fn cancel_swap_extrinsic(&self, request_id: String) -> Result<String> {
    self
      .inner
      .cancel_swap_extrinsic(wrapper::to_hash(request_id)?)
      .await
      .map_err(err_mapper)
  }

  #[napi]
  pub async fn batch_cancel_swaps(&self, request_ids: Vec<String>) -> Result<String> {
    let mut calls: Vec<TidechainCall> = vec![];

    for rid in request_ids.into_iter() {
      calls.push(TidechainCall::Tidefi(TidefiCall::CancelSwap {
        request_id: wrapper::to_hash(rid)?,
      }));
    }

    self
      .inner
      .force_batch_extrinsic(calls)
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
      self.inner.account_id().expect("No signer found").clone()
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
