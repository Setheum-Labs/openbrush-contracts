use crate::{
    governance::extensions::governor_counting::{
        CountingInternal,
        Data,
    },
    traits::{
        errors::GovernanceError,
        governance::ProposalId,
    },
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
    String,
};

pub trait GovernorCountingImpl: Storage<Data> + CountingInternal {
    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool {
        self.data::<Data>()
            .has_votes
            .get(&(proposal_id, account))
            .unwrap_or_default()
    }

    fn proposal_votes(&self, proposal_id: ProposalId) -> Result<(Balance, Balance, Balance), GovernanceError> {
        let proposal_vote = self
            .data::<Data>()
            .proposal_votes
            .get(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound)?;
        Ok((
            proposal_vote.for_votes,
            proposal_vote.against_votes,
            proposal_vote.abstain_votes,
        ))
    }
}
