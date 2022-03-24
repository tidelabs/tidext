use crate::{test_context, tidefi_keyring, AccountKeyring};
use frame_support::assert_ok;
use tidext::{assets::Asset, error::Error, Signer};

#[async_std::test]
async fn tx_basic_transfer() -> Result<(), Error> {
  // 10 tides
  let transfer_amount = Asset::Tide.saturating_mul(10);

  let alice = tidefi_keyring(AccountKeyring::Alice).await;
  let charlie = tidefi_keyring(AccountKeyring::Charlie).await;
  let cxt = test_context().await;
  let mut client = cxt.node_proc.client().clone();

  client.set_signer(charlie.clone());
  assert_eq!(client.signer.account_id(), charlie.account_id());

  let alice_pre = client
    .balance(&alice.account_id(), Asset::Tide.currency_id())
    .await?
    .available;

  let charlie_pre = client
    .balance(&charlie.account_id(), Asset::Tide.currency_id())
    .await?
    .available;

  assert_ok!(
    client
      .transfer_and_watch(
        &alice.account_id(),
        Asset::Tide.currency_id(),
        transfer_amount,
      )
      .await
  );

  let alice_post = client
    .balance(&alice.account_id(), Asset::Tide.currency_id())
    .await?
    .available;

  let charlie_post = client
    .balance(&charlie.account_id(), Asset::Tide.currency_id())
    .await?
    .available;

  assert!(charlie_pre - transfer_amount >= charlie_post);
  // initial deposit is 1 TIDE
  assert_eq!(alice_pre + transfer_amount - 1_000_000_000_000, alice_post);
  Ok(())
}
