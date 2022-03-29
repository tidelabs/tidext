use std::path::PathBuf;

use futures::future::TryFutureExt;
use pyo3::{
  exceptions::{PyConnectionError, PyIOError, PyRuntimeError},
  prelude::*,
  types::PyList,
};
use tidext::{
  primitives::AccountId,
  stronghold::{Location, ProcResult, Procedure, ResultMessage, Stronghold},
  Client as SubstrateClient, ClientBuilder as SubstrateClientBuilder, Error, Permill,
  TidefiKeyring,
};
use zeroize::Zeroize;

mod wrapper;

const SECRET_VAULT_PATH: &str = "TIDEFI";
const SR25519_KEYPAIR_RECORD_PATH: &str = "SR25519_KEYPAIR";

macro_rules! python_future {
  ($py: ident, $fut: expr) => {{
    pyo3_asyncio::tokio::future_into_py($py, async move {
      match $fut.await {
        Ok(r) => Python::with_gil(move |py| Ok(r.to_object(py))),
        Err(e) => Err(to_python_error(e)),
      }
    })
  }};
}

macro_rules! python_future_object {
  ($py: ident, $fut: expr) => {{
    pyo3_asyncio::tokio::future_into_py($py, async move {
      match $fut.await {
        Ok(r) => Python::with_gil(move |py| Ok(PyCell::new(py, r)?.to_object(py))),
        Err(e) => Err(to_python_error(e)),
      }
    })
  }};
}

fn to_python_error(error: Error) -> PyErr {
  match error {
    Error::Io(e) => PyIOError::new_err(e),
    Error::NetworkError(e) => PyConnectionError::new_err(e),
    _ => PyRuntimeError::new_err(error.to_string()),
  }
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

async fn get_signer(stronghold: Stronghold, keypair_location: Location) -> PyResult<TidefiKeyring> {
  let pk = match stronghold
    .runtime_exec(Procedure::Sr25519PublicKey {
      keypair: keypair_location.clone(),
    })
    .await
  {
    ProcResult::Sr25519PublicKey(ResultMessage::Ok(pk)) => pk,
    _ => return Err(PyRuntimeError::new_err("Failed to read public key")),
  };
  Ok(TidefiKeyring::new(
    AccountId::from(pk.inner().0),
    stronghold,
    keypair_location,
  ))
}

#[pyclass]
pub struct Currency {
  #[pyo3(get)]
  token_id: wrapper::CurrencyId,
  #[pyo3(get)]
  metadata: wrapper::CurrencyMetadata,
}

#[pyclass]
pub struct Builder {
  url: String,
  snapshot_path: String,
  password: String,
}

#[pymethods]
impl Builder {
  #[new]
  /// The client builder constructor.
  fn new(url: String, snapshot_path: String, password: String) -> Self {
    Self {
      url,
      snapshot_path,
      password,
    }
  }

  fn build<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
    let url = self.url.clone();
    let snapshot_path = self.snapshot_path.clone();
    let password = self.password.clone();
    pyo3_asyncio::tokio::future_into_py(py, async move {
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

      let snapshot_path = PathBuf::from(snapshot_path);
      if snapshot_path.exists() {
        let res = stronghold
          .read_snapshot(
            Vec::new(),
            None,
            &password_to_encryption_key(password.as_bytes().to_vec()).to_vec(),
            None,
            Some(snapshot_path),
          )
          .await;
        if let Err(e) = stronghold_response_to_result(res) {
          system.stop();
          return Err(PyIOError::new_err(e));
        }
      }

      let client = SubstrateClientBuilder::new()
        .set_signer(
          get_signer(
            stronghold,
            Location::generic(SECRET_VAULT_PATH, SR25519_KEYPAIR_RECORD_PATH),
          )
          .await?
          .pair_signer(),
        )
        .set_url(url)
        .build()
        .await
        .map(|c| Client { inner: c })
        .map_err(to_python_error)?;
      Python::with_gil(move |py| Py::new(py, client))
    })
  }
}

#[pyclass]
pub struct Client {
  inner: SubstrateClient,
}

#[pymethods]
impl Client {
  fn system_health<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future_object!(py, client.system_health().map_ok(wrapper::NodeHealth::from))
  }

  fn get_account_id(&self) -> Vec<u8> {
    let account_id: [u8; 32] = self.inner.account_id().clone().into();
    account_id.to_vec()
  }

  fn get_account_id_ss58(&self) -> String {
    self.inner.account_id().to_string()
  }

  /*
  FIXME: Expose Permill
  fn get_regular_swap_fee<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(py, client.swap_fee())
  }

  fn get_market_maker_swap_fee<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(py, client.swap_fee_market_maker())
  }
  */

  fn extrinsic_cost<'p>(&self, py: Python<'p>, extrinsic: String) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(py, client.extrinsic_cost(extrinsic))
  }

  fn submit_signed_extrinsic<'p>(&self, py: Python<'p>, extrinsic: String) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client
        .submit_signed_extrinsic(extrinsic)
        .map_ok(|hash| hash.0.to_vec())
    )
  }

  fn unstake_extrinsic<'p>(
    &self,
    py: Python<'p>,
    stake_id: Vec<u8>,
    force_unstake: bool,
  ) -> PyResult<&'p PyAny> {
    let stake_id: [u8; 32] = stake_id[0..32][..]
      .try_into()
      .expect("invalid stake id value");
    let client = self.inner.clone();
    python_future!(py, client.unstake_extrinsic(stake_id.into(), force_unstake))
  }

  fn stake_extrinsic<'p>(
    &self,
    py: Python<'p>,
    token_id: wrapper::CurrencyId,
    amount: u128,
    duration: u32,
  ) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client.stake_extrinsic(wrapper::currency_id_into(token_id), amount, duration,)
    )
  }

  fn swap_extrinsic<'p>(
    &self,
    py: Python<'p>,
    from_token_id: wrapper::CurrencyId,
    to_token_id: wrapper::CurrencyId,
    from_amount: u128,
    to_amount: u128,
    swap_type: wrapper::SwapType,
    slippage_tolerance: u32,
  ) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client.swap_extrinsic(
        wrapper::currency_id_into(from_token_id),
        from_amount,
        wrapper::currency_id_into(to_token_id),
        to_amount,
        swap_type.into(),
        Some(Permill::from_rational(slippage_tolerance, 1_000_000)),
      )
    )
  }

  fn transfer_extrinsic<'p>(
    &self,
    py: Python<'p>,
    token_id: wrapper::CurrencyId,
    amount: u128,
    destination: Vec<u8>,
  ) -> PyResult<&'p PyAny> {
    let destination: [u8; 32] = destination[0..32][..]
      .try_into()
      .expect("invalid destination value");
    let client = self.inner.clone();
    python_future!(
      py,
      client.transfer_extrinsic(
        AccountId::from(destination),
        wrapper::currency_id_into(token_id),
        amount,
      )
    )
  }

  fn balance<'p>(
    &self,
    py: Python<'p>,
    token_id: wrapper::CurrencyId,
    account_id: Option<Vec<u8>>,
  ) -> PyResult<&'p PyAny> {
    let account_id = if let Some(id) = account_id {
      let id: [u8; 32] = id[0..32][..].try_into().expect("invalid account id value");
      AccountId::from(id)
    } else {
      self.inner.account_id().clone()
    };

    let client = self.inner.clone();
    pyo3_asyncio::tokio::future_into_py(py, async move {
      let token_balance = client
        .balance(&account_id, wrapper::currency_id_into(token_id))
        .await
        .map_err(to_python_error)?;
      Python::with_gil(move |py| {
        Ok(
          PyCell::new(
            py,
            wrapper::CurrencyBalance {
              available: token_balance.available,
              reserved: token_balance.reserved,
            },
          )?
          .to_object(py),
        )
      })
    })
  }

  fn total_stake_for<'p>(
    &self,
    py: Python<'p>,
    token_id: wrapper::CurrencyId,
  ) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client.total_stake_for(wrapper::currency_id_into(token_id))
    )
  }

  fn total_supply_for<'p>(
    &self,
    py: Python<'p>,
    token_id: wrapper::CurrencyId,
  ) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client.total_supply_for(wrapper::currency_id_into(token_id))
    )
  }

  fn all_assets<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    pyo3_asyncio::tokio::future_into_py(py, async move {
      let assets = client.all_assets().await.map_err(to_python_error)?;
      Python::with_gil(move |py| {
        let mut py_assets = Vec::new();
        for (id, metadata) in assets {
          py_assets.push(PyCell::new(
            py,
            Currency {
              token_id: wrapper::currency_id_from(id),
              metadata: metadata.into(),
            },
          )?);
        }
        Ok(PyList::new(py, py_assets).to_object(py))
      })
    })
  }

  fn withdrawal<'p>(
    &self,
    py: Python<'p>,
    token_id: wrapper::CurrencyId,
    amount: u128,
    external_address: Vec<u8>,
  ) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client.withdrawal(
        wrapper::currency_id_into(token_id),
        amount,
        external_address,
      )
    )
  }

  fn withdrawal_extrinsic<'p>(
    &self,
    py: Python<'p>,
    token_id: wrapper::CurrencyId,
    amount: u128,
    external_address: Vec<u8>,
  ) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client.withdrawal_extrinsic(
        wrapper::currency_id_into(token_id),
        amount,
        external_address,
      )
    )
  }
}

/// A Python module implemented in Rust.
#[pymodule]
fn tidext(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_class::<Builder>()?;
  m.add_class::<Client>()?;

  Ok(())
}
