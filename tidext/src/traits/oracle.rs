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

use crate::{Client, Error};
use async_trait::async_trait;
use tidefi_primitives::{AccountId, Hash, OracleImAlive, SwapConfirmation};

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

  /// I'm alive
  async fn im_alive(&self, im_alive: OracleImAlive) -> Result<(), Error>;
}

#[async_trait]
impl OracleExt for Client {
  async fn im_alive(&self, im_alive: OracleImAlive) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .oracle()
      .im_alive(im_alive)?
      .sign_and_submit_default(self.signer()?)
      .await?;
    Ok(())
  }

  async fn confirm_swap(
    &self,
    request_id: Hash,
    market_makers: Vec<SwapConfirmation>,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .oracle()
      .confirm_swap(request_id, market_makers)?
      .sign_and_submit_default(self.signer()?)
      .await?;
    Ok(())
  }

  async fn update_status(&self, enabled: bool) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .oracle()
      .set_status(enabled)?
      .sign_and_submit_default(self.signer()?)
      .await?;
    Ok(())
  }

  async fn update_account_id(&self, account_id: &AccountId) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .oracle()
      .set_account_id(account_id.clone())?
      .sign_and_submit_default(self.signer()?)
      .await?;
    Ok(())
  }

  async fn cancel_swap(&self, request_id: Hash) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .oracle()
      .cancel_swap(request_id)?
      .sign_and_submit_default(self.signer()?)
      .await?;
    Ok(())
  }

  async fn add_market_maker(&self, account_id: &AccountId) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .oracle()
      .add_market_maker(account_id.clone())?
      .sign_and_submit_default(self.signer()?)
      .await?;
    Ok(())
  }

  async fn remove_market_maker(&self, account_id: &AccountId) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .oracle()
      .remove_market_maker(account_id.clone())?
      .sign_and_submit_default(self.signer()?)
      .await?;
    Ok(())
  }
}
