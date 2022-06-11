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
  stronghold::{Location, ResultMessage},
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
    let stronghold_path = self.snapshot_path.clone().into();
    let location = Location::generic(SECRET_VAULT_PATH, SR25519_KEYPAIR_RECORD_PATH);
    let mut password = password_to_encryption_key(self.password.as_bytes().to_vec()).to_vec();
    pyo3_asyncio::tokio::future_into_py(py, async move {
      let client = try_build(&url, stronghold_path, location, &password)
        .await
        .map(|client| {
          password.zeroize();
          client
        })
        .map_err(|e| {
          password.zeroize();
          to_python_error(e)
        })?;
      Python::with_gil(move |py| Py::new(py, client))
    })
  }
}

async fn try_build(
  url: &str,
  stronghold_path: PathBuf,
  location: Location,
  password: &Vec<u8>,
) -> Result<Client, Error> {
  let builder = SubstrateClientBuilder::new()
    .set_signer(if stronghold_path.exists() {
      TidefiKeyring::try_from_stronghold_path(&stronghold_path, Some(location), Some(&password))
        .await?
    } else {
      let mut stronghold = init_stronghold_from_seed(&location, None, None).await?;
      let res = stronghold
        .write_all_to_snapshot(password, None, Some(stronghold_path))
        .await;
      if let Err(e) = stronghold_response_to_result(res) {
        return Err(Error::Stronghold(e));
      }

      TidefiKeyring::try_from_stronghold_instance(stronghold, Some(location)).await?
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
    self
      .inner
      .account_id()
      .expect("No signer found")
      .to_string()
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
