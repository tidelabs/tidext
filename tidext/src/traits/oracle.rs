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

use crate::{with_runtime, Client, Error};
use async_trait::async_trait;
use tidefi_primitives::{AccountId, AssetId, Balance, Hash, SwapConfirmation};

/// An extension trait for `Client` that provides a variety of convenient Oracle functions.
#[async_trait]
pub trait OracleExt {
  /// Update oracle status
  async fn update_status(&self, enabled: bool) -> Result<(), Error>;

  /// Update oracle account id
  async fn update_account_id(&self, account_id: &AccountId) -> Result<(), Error>;

  /// Add account id as a new trusted market maker
  async fn add_market_maker(&self, account_id: &AccountId) -> Result<(), Error>;

  /// Remove account id from the trusted market maker list
  async fn remove_market_maker(&self, account_id: &AccountId) -> Result<(), Error>;

  /// Confirm swap
  async fn confirm_swap(
    &self,
    request_id: Hash,
    market_makers: Vec<SwapConfirmation>,
  ) -> Result<(), Error>;

  /// Cancel swap
  async fn cancel_swap(&self, request_id: Hash) -> Result<(), Error>;

  /// Update assets value for the sunrise pool
  /// It should represent how many TDFY's for 1 `AssetId`.
  ///
  /// The value should always be formatted with TDFY decimals (12)
  ///
  /// ## Example:
  ///
  /// If the Bitcoin price is 0.001815 BTC (for 1 TDFY)
  /// You get 550.9641873278 TDFY for 1 BTC
  ///
  /// The value should be: `vec![(2, 550_964_187_327_800)]`
  ///
  /// ***
  ///
  /// If the ETH price is 0.03133 ETH (for 1 TDFY)
  /// You get 31.9182891796999 TDFY for 1 ETH
  ///
  /// The value sent should be: `vec![(4, 31_918_289_179_699)]`
  ///
  /// ***
  ///
  /// If the USDT price is 33.650000 USDT (for 1 TDFY)
  /// You get 0.029717682000 TDFY for 1 USDT
  ///
  /// The value sent should be: `vec![(4, 29_717_682_020)]`
  async fn update_assets_value(&self, values: Vec<(AssetId, Balance)>) -> Result<(), Error>;
}

#[async_trait]
impl OracleExt for Client {
  async fn update_assets_value(&self, values: Vec<(AssetId, Balance)>) -> Result<(), Error> {
    with_runtime! {
      self,
      current_runtime,
      {
        self
        .runtime()
        .tx()
        .sign_and_submit_default(&current_runtime.tx().oracle().update_assets_value(values), self.signer()?)
        .await?
      }
    };
    Ok(())
  }

  async fn confirm_swap(
    &self,
    request_id: Hash,
    market_makers: Vec<SwapConfirmation>,
  ) -> Result<(), Error> {
    with_runtime! {
      self,
      current_runtime,
      {
        self
        .runtime()
        .tx()
        .sign_and_submit_default(&current_runtime.tx().oracle().confirm_swap(request_id, market_makers), self.signer()?)
        .await?
      }
    };
    Ok(())
  }

  async fn update_status(&self, enabled: bool) -> Result<(), Error> {
    with_runtime! {
      self,
      current_runtime,
      {
        self
        .runtime()
        .tx()
        .sign_and_submit_default(&current_runtime.tx().oracle().set_status(enabled), self.signer()?)
        .await?
      }
    };
    Ok(())
  }

  async fn update_account_id(&self, account_id: &AccountId) -> Result<(), Error> {
    with_runtime! {
      self,
      current_runtime,
      {
        self
        .runtime()
        .tx()
        .sign_and_submit_default(&current_runtime.tx().oracle().set_account_id(account_id.clone()), self.signer()?)
        .await?
      }
    };
    Ok(())
  }

  async fn cancel_swap(&self, request_id: Hash) -> Result<(), Error> {
    with_runtime! {
      self,
      current_runtime,
      {
        self
        .runtime()
        .tx()
        .sign_and_submit_default(&current_runtime.tx().oracle().cancel_swap(request_id), self.signer()?)
        .await?
      }
    };
    Ok(())
  }

  async fn add_market_maker(&self, account_id: &AccountId) -> Result<(), Error> {
    with_runtime! {
      self,
      current_runtime,
      {
        self
        .runtime()
        .tx()
        .sign_and_submit_default(&current_runtime.tx().oracle().add_market_maker(account_id.clone()), self.signer()?)
        .await?
      }
    };
    Ok(())
  }

  async fn remove_market_maker(&self, account_id: &AccountId) -> Result<(), Error> {
    with_runtime! {
      self,
      current_runtime,
      {
        self
        .runtime()
        .tx()
        .sign_and_submit_default(&current_runtime.tx().oracle().remove_market_maker(account_id.clone()), self.signer()?)
        .await?
      }
    };
    Ok(())
  }
}
