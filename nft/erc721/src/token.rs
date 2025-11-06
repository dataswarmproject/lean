//! ERC-721 token implementation

use crate::{NFTError, Result, TokenMetadata, ApprovalManager};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Token ID type
pub type TokenId = u128;

/// NFT Contract following ERC-721 standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTContract {
    /// Contract name
    pub name: String,
    /// Contract symbol
    pub symbol: String,
    /// Base URI for token metadata
    pub base_uri: String,
    /// Token ownership: token_id -> owner
    owners: HashMap<TokenId, Address>,
    /// Owner balances: owner -> count
    balances: HashMap<Address, u128>,
    /// Token metadata: token_id -> metadata
    metadata: HashMap<TokenId, TokenMetadata>,
    /// Approval manager
    approvals: ApprovalManager,
    /// Total supply
    total_supply: u128,
}

impl NFTContract {
    /// Create a new NFT contract
    pub fn new(name: String, symbol: String, base_uri: String) -> Self {
        Self {
            name,
            symbol,
            base_uri,
            owners: HashMap::new(),
            balances: HashMap::new(),
            metadata: HashMap::new(),
            approvals: ApprovalManager::new(),
            total_supply: 0,
        }
    }

    /// Get total supply
    pub fn total_supply(&self) -> u128 {
        self.total_supply
    }

    /// Get owner of token
    pub fn owner_of(&self, token_id: TokenId) -> Result<Address> {
        self.owners
            .get(&token_id)
            .copied()
            .ok_or(NFTError::TokenNotFound(token_id))
    }

    /// Get balance of owner
    pub fn balance_of(&self, owner: &Address) -> u128 {
        self.balances.get(owner).copied().unwrap_or(0)
    }

    /// Check if token exists
    pub fn exists(&self, token_id: TokenId) -> bool {
        self.owners.contains_key(&token_id)
    }

    /// Mint new token
    pub fn mint(
        &mut self,
        to: Address,
        token_id: TokenId,
        metadata: TokenMetadata,
    ) -> Result<()> {
        // Check if recipient is valid
        if to == Address::zero() {
            return Err(NFTError::InvalidRecipient);
        }

        // Check if token already exists
        if self.exists(token_id) {
            return Err(NFTError::TokenAlreadyExists(token_id));
        }

        // Mint token
        self.owners.insert(token_id, to);
        self.metadata.insert(token_id, metadata);
        
        // Update balance
        let balance = self.balances.entry(to).or_insert(0);
        *balance = balance
            .checked_add(1)
            .ok_or(NFTError::ArithmeticOverflow)?;

        // Update total supply
        self.total_supply = self
            .total_supply
            .checked_add(1)
            .ok_or(NFTError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Burn token
    pub fn burn(&mut self, token_id: TokenId, caller: &Address) -> Result<()> {
        let owner = self.owner_of(token_id)?;

        // Check authorization
        if &owner != caller && !self.approvals.is_approved_for_all(&owner, caller) {
            return Err(NFTError::NotAuthorized(hex::encode(caller.as_bytes())));
        }

        // Remove token
        self.owners.remove(&token_id);
        self.metadata.remove(&token_id);
        self.approvals.clear_approval(token_id);

        // Update balance
        if let Some(balance) = self.balances.get_mut(&owner) {
            *balance = balance.saturating_sub(1);
        }

        // Update total supply
        self.total_supply = self.total_supply.saturating_sub(1);

        Ok(())
    }

    /// Transfer token
    pub fn transfer_from(
        &mut self,
        from: &Address,
        to: &Address,
        token_id: TokenId,
        caller: &Address,
    ) -> Result<()> {
        // Validate recipient
        if *to == Address::zero() {
            return Err(NFTError::InvalidRecipient);
        }

        if from == to {
            return Err(NFTError::TransferToSelf);
        }

        // Check ownership
        let owner = self.owner_of(token_id)?;
        if &owner != from {
            return Err(NFTError::NotOwner {
                token_id,
                owner: hex::encode(owner.as_bytes()),
                caller: hex::encode(from.as_bytes()),
            });
        }

        // Check authorization
        if caller != from
            && !self.approvals.is_approved(token_id, caller)
            && !self.approvals.is_approved_for_all(from, caller)
        {
            return Err(NFTError::NotAuthorized(hex::encode(caller.as_bytes())));
        }

        // Transfer ownership
        self.owners.insert(token_id, *to);

        // Update balances
        if let Some(from_balance) = self.balances.get_mut(from) {
            *from_balance = from_balance.saturating_sub(1);
        }
        let to_balance = self.balances.entry(*to).or_insert(0);
        *to_balance = to_balance
            .checked_add(1)
            .ok_or(NFTError::ArithmeticOverflow)?;

        // Clear approvals
        self.approvals.clear_approval(token_id);

        Ok(())
    }

    /// Safe transfer (includes receiver check in production)
    pub fn safe_transfer_from(
        &mut self,
        from: &Address,
        to: &Address,
        token_id: TokenId,
        caller: &Address,
    ) -> Result<()> {
        self.transfer_from(from, to, token_id, caller)?;
        // In production: check if recipient is contract and call onERC721Received
        Ok(())
    }

    /// Approve address to transfer token
    pub fn approve(&mut self, approved: Address, token_id: TokenId, caller: &Address) -> Result<()> {
        let owner = self.owner_of(token_id)?;

        if caller != &owner && !self.approvals.is_approved_for_all(&owner, caller) {
            return Err(NFTError::NotAuthorized(hex::encode(caller.as_bytes())));
        }

        self.approvals.set_approval(token_id, approved);
        Ok(())
    }

    /// Set approval for all tokens
    pub fn set_approval_for_all(&mut self, owner: Address, operator: Address, approved: bool) {
        self.approvals.set_approval_for_all(owner, operator, approved);
    }

    /// Get approved address for token
    pub fn get_approved(&self, token_id: TokenId) -> Option<Address> {
        self.approvals.get_approved(token_id)
    }

    /// Check if operator is approved for all
    pub fn is_approved_for_all(&self, owner: &Address, operator: &Address) -> bool {
        self.approvals.is_approved_for_all(owner, operator)
    }

    /// Get token metadata
    pub fn token_metadata(&self, token_id: TokenId) -> Result<&TokenMetadata> {
        self.metadata
            .get(&token_id)
            .ok_or(NFTError::TokenNotFound(token_id))
    }

    /// Get token URI
    pub fn token_uri(&self, token_id: TokenId) -> Result<String> {
        if !self.exists(token_id) {
            return Err(NFTError::TokenNotFound(token_id));
        }
        Ok(format!("{}{}", self.base_uri, token_id))
    }

    /// Get all tokens owned by address
    pub fn tokens_of_owner(&self, owner: &Address) -> Vec<TokenId> {
        self.owners
            .iter()
            .filter_map(|(id, addr)| if addr == owner { Some(*id) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address(n: u8) -> Address {
        Address::from([n; 20])
    }

    fn test_metadata() -> TokenMetadata {
        TokenMetadata {
            name: "Test NFT".to_string(),
            description: "A test NFT".to_string(),
            image: "https://example.com/image.png".to_string(),
            attributes: vec![],
        }
    }

    #[test]
    fn test_mint() {
        let mut nft = NFTContract::new(
            "Test NFT".to_string(),
            "TNFT".to_string(),
            "https://example.com/".to_string(),
        );

        nft.mint(test_address(1), 1, test_metadata()).unwrap();
        
        assert_eq!(nft.total_supply(), 1);
        assert_eq!(nft.owner_of(1).unwrap(), test_address(1));
        assert_eq!(nft.balance_of(&test_address(1)), 1);
    }

    #[test]
    fn test_transfer() {
        let mut nft = NFTContract::new(
            "Test NFT".to_string(),
            "TNFT".to_string(),
            "https://example.com/".to_string(),
        );

        nft.mint(test_address(1), 1, test_metadata()).unwrap();
        
        nft.transfer_from(
            &test_address(1),
            &test_address(2),
            1,
            &test_address(1),
        ).unwrap();

        assert_eq!(nft.owner_of(1).unwrap(), test_address(2));
        assert_eq!(nft.balance_of(&test_address(1)), 0);
        assert_eq!(nft.balance_of(&test_address(2)), 1);
    }

    #[test]
    fn test_approval() {
        let mut nft = NFTContract::new(
            "Test NFT".to_string(),
            "TNFT".to_string(),
            "https://example.com/".to_string(),
        );

        nft.mint(test_address(1), 1, test_metadata()).unwrap();
        nft.approve(test_address(2), 1, &test_address(1)).unwrap();

        assert_eq!(nft.get_approved(1), Some(test_address(2)));

        // Approved address can transfer
        nft.transfer_from(
            &test_address(1),
            &test_address(3),
            1,
            &test_address(2),
        ).unwrap();

        assert_eq!(nft.owner_of(1).unwrap(), test_address(3));
    }

    #[test]
    fn test_burn() {
        let mut nft = NFTContract::new(
            "Test NFT".to_string(),
            "TNFT".to_string(),
            "https://example.com/".to_string(),
        );

        nft.mint(test_address(1), 1, test_metadata()).unwrap();
        nft.burn(1, &test_address(1)).unwrap();

        assert_eq!(nft.total_supply(), 0);
        assert!(!nft.exists(1));
    }
}
