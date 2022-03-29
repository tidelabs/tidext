use simple_logger::SimpleLogger;
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
  SimpleLogger::new()
    .with_module_level("subxt", log::LevelFilter::Off)
    .with_module_level("jsonrpsee_ws_client", log::LevelFilter::Off)
    .with_module_level("soketto", log::LevelFilter::Off)
    .with_module_level("tracing", log::LevelFilter::Off)
    .with_module_level("mio", log::LevelFilter::Off)
    .init()?;
  Ok(())
}
