use simple_logger::SimpleLogger;
use tidext::keyring::{
  stronghold::{Location, ProcResult, Procedure, ResultMessage, Stronghold},
  AccountId, TidefiKeyring, TidefiPairSigner,
};

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

pub async fn init_signer(seed: String) -> TidefiPairSigner {
  let (account_id, location, stronghold) = init_account(seed, None).await;
  TidefiKeyring::new(account_id, stronghold, location).pair_signer()
}

pub async fn init_account(
  seed: String,
  passphrase: Option<String>,
) -> (AccountId, Location, Stronghold) {
  let (tx, rx) = std::sync::mpsc::channel();
  std::thread::spawn(move || {
    let system = actix::System::new();
    let stronghold = system
      .block_on(Stronghold::init_stronghold_system(b"path".to_vec(), vec![]))
      .unwrap();
    tx.send(stronghold).unwrap();
    system.run().expect("actix system run failed");
  });
  let stronghold = rx.recv().unwrap();

  let keypair_location = Location::generic("SR25519", "keypair");

  match stronghold
    .runtime_exec(Procedure::Sr25519Generate {
      mnemonic_or_seed: Some(seed),
      passphrase,
      output: keypair_location.clone(),
      hint: [0u8; 24].into(),
    })
    .await
  {
    ProcResult::Sr25519Generate(ResultMessage::OK) => (),
    r => panic!("unexpected result: {:?}", r),
  }

  let pk = match stronghold
    .runtime_exec(Procedure::Sr25519PublicKey {
      keypair: keypair_location.clone(),
    })
    .await
  {
    ProcResult::Sr25519PublicKey(ResultMessage::Ok(pk)) => pk,
    r => panic!("unexpected result: {:?}", r),
  };

  (AccountId::from(pk.inner().0), keypair_location, stronghold)
}
