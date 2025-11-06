//! Transaction types for LEAN CHAIN

use lean_crypto::{Address, Hash256};
use serde::{Deserialize, Serialize};

/// All supported transaction types
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    /// Simple payment transaction (P2P transfer)
    Payment(PaymentTransaction),

    /// Deploy a new smart contract
    ContractDeploy(ContractDeployTransaction),

    /// Call an existing smart contract
    ContractCall(ContractCallTransaction),

    /// Stake tokens to become a validator
    Stake(StakeTransaction),

    /// Unstake validator tokens
    Unstake(UnstakeTransaction),

    /// Delegate tokens to a validator
    Delegate(DelegateTransaction),

    /// Submit a governance proposal
    GovernanceProposal(GovernanceProposalTransaction),

    /// Vote on a governance proposal
    GovernanceVote(GovernanceVoteTransaction),

    /// Cross-chain bridge operation
    BridgeTransfer(BridgeTransferTransaction),

    /// KYC/AML compliance attestation
    ComplianceAttestation(ComplianceAttestationTransaction),
}

/// Simple payment transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaymentTransaction {
    /// Recipient address
    pub to: Address,

    /// Amount to transfer (in smallest unit)
    pub amount: u128,
}

/// Contract deployment transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractDeployTransaction {
    /// Contract bytecode
    pub code: Vec<u8>,

    /// Constructor arguments
    pub constructor_args: Vec<u8>,

    /// Initial value to send to contract
    pub value: u128,
}

/// Contract call transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractCallTransaction {
    /// Contract address to call
    pub to: Address,

    /// Function call data (encoded)
    pub data: Vec<u8>,

    /// Value to send with call
    pub value: u128,
}

/// Stake transaction (become a validator)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StakeTransaction {
    /// Amount to stake (minimum 100,000 LEAN)
    pub amount: u128,

    /// Validator public key
    pub validator_key: Vec<u8>,

    /// Commission rate (percentage, 0-100)
    pub commission_rate: u8,
}

/// Unstake transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnstakeTransaction {
    /// Amount to unstake
    pub amount: u128,

    /// Validator address
    pub validator: Address,
}

/// Delegate transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DelegateTransaction {
    /// Validator to delegate to
    pub validator: Address,

    /// Amount to delegate
    pub amount: u128,
}

/// Governance proposal transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceProposalTransaction {
    /// Proposal title
    pub title: String,

    /// Proposal description
    pub description: String,

    /// Proposal type
    pub proposal_type: ProposalType,

    /// Proposal data (encoded based on type)
    pub data: Vec<u8>,

    /// Voting period (in blocks)
    pub voting_period: u64,
}

/// Types of governance proposals
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalType {
    /// Parameter change proposal
    ParameterChange,

    /// Protocol upgrade proposal
    ProtocolUpgrade,

    /// Treasury spend proposal
    TreasurySpend,

    /// Validator addition/removal
    ValidatorChange,

    /// General text proposal
    TextProposal,
}

/// Governance vote transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceVoteTransaction {
    /// Proposal ID
    pub proposal_id: u64,

    /// Vote choice
    pub vote: VoteChoice,

    /// Voting power (calculated from stake)
    pub voting_power: u128,
}

/// Vote choices
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
    NoWithVeto,
}

/// Cross-chain bridge transfer
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BridgeTransferTransaction {
    /// Destination chain ID
    pub dest_chain_id: u64,

    /// Destination address (on target chain)
    pub dest_address: Vec<u8>,

    /// Token address (if ERC-20 or similar)
    pub token: Option<Address>,

    /// Amount to bridge
    pub amount: u128,

    /// Bridge fee
    pub bridge_fee: u128,
}

/// Compliance attestation (optional KYC/AML)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComplianceAttestationTransaction {
    /// Attestation type
    pub attestation_type: AttestationType,

    /// Attestation provider address
    pub provider: Address,

    /// Attestation data hash (privacy-preserving)
    pub data_hash: Hash256,

    /// Expiration timestamp
    pub expiration: i64,

    /// Signature from compliance provider
    pub provider_signature: Vec<u8>,
}

/// Types of compliance attestations
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttestationType {
    /// Know Your Customer
    KYC,

    /// Anti-Money Laundering
    AML,

    /// Accredited Investor
    AccreditedInvestor,

    /// Jurisdiction Verification
    Jurisdiction,

    /// Custom attestation
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_transaction() {
        let payment = PaymentTransaction {
            to: Address::zero(),
            amount: 1_000_000,
        };

        let tx_type = TransactionType::Payment(payment);
        assert!(matches!(tx_type, TransactionType::Payment(_)));
    }

    #[test]
    fn test_stake_transaction() {
        let stake = StakeTransaction {
            amount: 100_000_000_000, // 100,000 LEAN
            validator_key: vec![0u8; 32],
            commission_rate: 10,
        };

        let tx_type = TransactionType::Stake(stake);
        assert!(matches!(tx_type, TransactionType::Stake(_)));
    }

    #[test]
    fn test_governance_vote() {
        let vote = GovernanceVoteTransaction {
            proposal_id: 1,
            vote: VoteChoice::Yes,
            voting_power: 1_000_000,
        };

        let tx_type = TransactionType::GovernanceVote(vote);
        assert!(matches!(tx_type, TransactionType::GovernanceVote(_)));
    }
}
