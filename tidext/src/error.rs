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

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("utf8 error: {0}")]
  Utf8(#[from] std::string::FromUtf8Error),
  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("tidechain error: {0}")]
  Substrate(#[from] subxt::Error),
  #[error("dispatch error: {0}")]
  DispatchError(String),
  #[error("codec error: {0}")]
  Codec(#[from] parity_scale_codec::Error),
  #[error("jsonrpsee error: {0}")]
  JsonRpsee(#[from] jsonrpsee::core::Error),
  #[error("other error: {0}")]
  Other(String),
  #[error("stronghold error: {0}")]
  Stronghold(String),
  #[cfg(feature = "keyring-stronghold")]
  #[error("stronghold engine error: {0}")]
  StrongholdClient(#[from] iota_stronghold::types::ClientError),
  #[cfg(feature = "keyring-stronghold")]
  #[error("stronghold procedure error: {0}")]
  StrongholdProcedure(#[from] iota_stronghold::procedures::ProcedureError),
  #[cfg(feature = "keyring-stronghold_ex")]
  #[error("stronghold engine error: {0}")]
  ClientExError(#[from] ex_client::Error),
  #[error("quorum: {0}")]
  QuorumInit(String),
  #[error("network error: {0}")]
  NetworkError(String),
  #[error("channel closed")]
  ChannelClosed,
  #[error("No signer available, use `set_signer()` first")]
  NoSignerAvailable,
}

impl From<&str> for Error {
  fn from(err: &str) -> Self {
    Error::Other(err.to_string())
  }
}
impl From<String> for Error {
  fn from(err: String) -> Self {
    Error::Other(err)
  }
}
