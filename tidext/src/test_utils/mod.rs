mod context;
mod node_proc;
pub use crate::keyring::{
  stronghold::{Location, ProcResult, Procedure, ResultMessage, Stronghold},
  AccountId, TidefiKeyring, TidefiPairSigner,
};
pub use sp_keyring::AccountKeyring;

pub use context::*;
pub use node_proc::TestNodeProcess;

pub async fn tidefi_keyring(account: AccountKeyring) -> TidefiPairSigner {
  let (account_id, location, stronghold) = init_account(account.to_seed(), None).await;
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
