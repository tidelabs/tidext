#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("utf8 error: {0}")]
  Utf8(#[from] std::string::FromUtf8Error),
  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("substrate error: {0}")]
  Substrate(#[from] subxt::BasicError),
  #[error("substrate error: {0}")]
  Codec(#[from] parity_scale_codec::Error),
  #[error("other error: {0}")]
  Other(String),
  #[error("quorum initialization error: {0}")]
  QuorumInit(String),
  #[error("network error: {0}")]
  NetworkError(String),
  #[error("subscription channel closed")]
  ChannelClosed,
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
