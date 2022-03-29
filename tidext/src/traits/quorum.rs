use crate::{Client, Error};
use async_trait::async_trait;
use tidefi_primitives::{
  AccountId, AssetId, BlockNumber, Hash, ProposalType, ProposalVotes, Withdrawal,
};
/// An extension trait for `Client` that provides a variety of convenient Quorum functions.
#[async_trait]
pub trait QuorumExt {
  /// Get all proposals
  /// return Vec<(proposal_id, initial_block, proposal_type)>
  async fn get_proposals(
    &self,
  ) -> Result<
    Vec<(
      Hash,
      BlockNumber,
      ProposalType<AccountId, BlockNumber, Vec<u8>, Vec<AccountId>>,
    )>,
    Error,
  >;

  /// Get all pending burned
  async fn get_burned_queue(
    &self,
  ) -> Result<Vec<(Hash, Withdrawal<AccountId, BlockNumber, Vec<u8>>)>, Error>;

  /// Get proposal votes and status
  async fn get_proposal(
    &self,
    proposal: Hash,
  ) -> Result<Option<ProposalVotes<BlockNumber, Vec<AccountId>>>, Error>;

  /// Submit new proposal
  async fn submit_proposal(
    &self,
    proposal: ProposalType<AccountId, BlockNumber, Vec<u8>, Vec<AccountId>>,
  ) -> Result<(), Error>;

  /// Acknowledge a proposal
  async fn acknowledge_proposal(&self, proposal: Hash) -> Result<(), Error>;

  /// Acknowledge a burned item, and remove it from the queue
  async fn acknowledge_burned(&self, proposal: Hash) -> Result<(), Error>;

  /// Reject a proposal
  async fn reject_proposal(&self, proposal: Hash) -> Result<(), Error>;

  /// Submit public keys for the current signer (required) for each members and wait finalization on-chain
  ///
  /// You need to submit all public keys at once as all previous keys will be deleted for this account.
  ///
  /// When a new configuration is set, all members need to set their public keys before they can submit,
  /// acknowledge and reject any proposal.
  async fn submit_public_keys(&self, public_keys: Vec<(AssetId, Vec<u8>)>) -> Result<(), Error>;

  /// Validate if an account id is currently part of the quorum set
  async fn is_quorum_member(&self, account_id: &AccountId) -> Result<bool, Error>;
}

#[async_trait]
impl QuorumExt for Client {
  async fn get_proposals(
    &self,
  ) -> Result<
    Vec<(
      Hash,
      BlockNumber,
      ProposalType<AccountId, BlockNumber, Vec<u8>, Vec<AccountId>>,
    )>,
    Error,
  > {
    let proposals = self.runtime().storage().quorum().proposals(None).await?;
    Ok(proposals)
  }

  async fn get_burned_queue(
    &self,
  ) -> Result<Vec<(Hash, Withdrawal<AccountId, BlockNumber, Vec<u8>>)>, Error> {
    let proposals = self.runtime().storage().quorum().burned_queue(None).await?;
    Ok(proposals)
  }

  async fn get_proposal(
    &self,
    proposal: Hash,
  ) -> Result<Option<ProposalVotes<BlockNumber, Vec<AccountId>>>, Error> {
    let proposals = self
      .runtime()
      .storage()
      .quorum()
      .votes(&proposal, None)
      .await?;
    Ok(proposals)
  }

  async fn submit_proposal(
    &self,
    proposal: ProposalType<AccountId, BlockNumber, Vec<u8>, Vec<AccountId>>,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .quorum()
      .submit_proposal(proposal)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  async fn acknowledge_proposal(&self, proposal: Hash) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .quorum()
      .acknowledge_proposal(proposal)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  async fn acknowledge_burned(&self, proposal: Hash) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .quorum()
      .acknowledge_burned(proposal)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  async fn reject_proposal(&self, proposal: Hash) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .quorum()
      .reject_proposal(proposal)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  async fn submit_public_keys(&self, public_keys: Vec<(AssetId, Vec<u8>)>) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .quorum()
      .submit_public_keys(public_keys)
      .sign_and_submit_then_watch(&self.signer)
      .await?
      .wait_for_finalized_success()
      .await
      .map_err(|err| Error::QuorumInit(err.to_string()))?;
    Ok(())
  }

  async fn is_quorum_member(&self, account_id: &AccountId) -> Result<bool, Error> {
    let is_member = self
      .runtime()
      .storage()
      .quorum()
      .members(account_id, None)
      .await?
      .unwrap_or(false);
    Ok(is_member)
  }
}
