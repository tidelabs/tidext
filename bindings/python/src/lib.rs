// Copyright 2021-2023 Semantic Network Ltd.
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

use std::path::PathBuf;

use futures::future::TryFutureExt;
use pyo3::{
  exceptions::{PyConnectionError, PyIOError, PyRuntimeError},
  prelude::*,
  types::PyList,
};
use tidext::{
  init_stronghold_from_seed,
  primitives::AccountId,
  stronghold::{ClientError, KeyProvider, Location, SnapshotPath},
  Client as SubstrateClient, ClientBuilder as SubstrateClientBuilder, Error, Permill,
  TidechainCall, TidefiCall, TidefiKeyring,
};
use zeroize::Zeroize;

mod wrapper;

const SECRET_VAULT_PATH: &str = "TIDEFI";
const SR25519_KEYPAIR_RECORD_PATH: &str = "SR25519_KEYPAIR";
const SECRET_KEY_FILE_EXTENSION: &str = "secret-key";
const SECRET_KEY_LENGTH: usize = 128;

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
  client_path: Vec<u8>,
  snapshot_path: String,
  password: String,
}

#[pymethods]
impl Builder {
  #[new]
  /// The client builder constructor.
  fn new(url: String, client_path: Vec<u8>, snapshot_path: String, password: String) -> Self {
    Self {
      client_path,
      url,
      snapshot_path,
      password,
    }
  }

  fn build<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
    let url = self.url.clone();
    let stronghold_path = self.snapshot_path.clone().into();
    let client_path = self.client_path.clone();
    let location = Location::generic(SECRET_VAULT_PATH, SR25519_KEYPAIR_RECORD_PATH);
    let password = self.password.clone();
    pyo3_asyncio::tokio::future_into_py(py, async move {
      let client = try_build(&url, stronghold_path, client_path, location, &password)
        .await
        .map_err(to_python_error)?;
      Python::with_gil(move |py| Py::new(py, client))
    })
  }
}

async fn try_build(
  url: &str,
  stronghold_path: PathBuf,
  client_path: Vec<u8>,
  location: Location,
  password: &str,
) -> Result<Client, Error> {
  let secret_key_path = stronghold_path
    .as_path()
    .with_extension(SECRET_KEY_FILE_EXTENSION);

  let builder = SubstrateClientBuilder::new()
    .set_signer(if stronghold_path.exists() {
      let secret_key = std::fs::read(secret_key_path)?;
      TidefiKeyring::try_from_stronghold_path(
        client_path.clone(),
        &stronghold_path,
        Some(location),
        Some(hash_password(password.as_bytes().to_vec(), secret_key)),
      )?
    } else {
      let mut secret_key = [0u8; SECRET_KEY_LENGTH];
      crypto::utils::rand::fill(&mut secret_key).map_err(|e| Error::Other(e.to_string()))?;

      let stronghold = init_stronghold_from_seed(client_path.clone(), &location, None, None)?;

      let snapshot_path = SnapshotPath::named(stronghold_path);
      let key_provider = KeyProvider::try_from(hash_password(
        password.as_bytes().to_vec(),
        secret_key.to_vec(),
      ))
      .map_err(|e| {
        ClientError::Provider(format!("Couldn't build a key from the password: {:?}", e))
      })?;

      stronghold.commit_with_keyprovider(&snapshot_path, &key_provider)?;
      std::fs::write(secret_key_path, secret_key)?;

      TidefiKeyring::try_from_stronghold_instance(client_path, stronghold, Some(location))?
    })
    .set_url(url);

  Ok(Client {
    inner: builder.build().await?,
  })
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
    let account_id: [u8; 32] = self
      .inner
      .account_id()
      .expect("No signer found")
      .clone()
      .into();
    account_id.to_vec()
  }

  fn get_account_id_ss58(&self) -> String {
    self.inner.account_id_ss58().expect("No signer found")
  }

  fn get_regular_swap_fee<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(py, client.swap_fee().map_ok(|a| a.deconstruct()))
  }

  fn get_market_maker_swap_fee<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client.swap_fee_market_maker().map_ok(|a| a.deconstruct())
    )
  }

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
        .map_ok(wrapper::hash_to_string)
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

  #[allow(clippy::too_many_arguments)]
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

  fn cancel_swap_extrinsic<'p>(&self, py: Python<'p>, request_id: String) -> PyResult<&'p PyAny> {
    let client = self.inner.clone();
    python_future!(
      py,
      client.cancel_swap_extrinsic(wrapper::to_hash(request_id)?)
    )
  }

  fn batch_cancel_swaps_extrinsic<'p>(
    &self,
    py: Python<'p>,
    request_ids: Vec<String>,
  ) -> PyResult<&'p PyAny> {
    let mut calls: Vec<TidechainCall> = vec![];
    let client = self.inner.clone();

    for rid in request_ids.into_iter() {
      calls.push(TidechainCall::Tidefi(TidefiCall::CancelSwap {
        request_id: wrapper::to_hash(rid)?,
      }));
    }

    python_future!(py, client.force_batch_extrinsic(calls))
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
      self.inner.account_id().expect("No signer found").clone()
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
