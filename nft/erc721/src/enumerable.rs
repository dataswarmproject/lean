//! ERC-721 Enumerable extension

use crate::{TokenId, Result};
use lean_crypto::Address;
use std::collections::HashMap;

/// Token iterator
pub struct TokenIterator {
    tokens: Vec<TokenId>,
    index: usize,
}

impl Iterator for TokenIterator {
    type Item = TokenId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            let token = self.tokens[self.index];
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }
}

/// Enumerable NFT extension
pub struct EnumerableNFT {
    /// All token IDs
    all_tokens: Vec<TokenId>,
    /// Token index in all_tokens
    all_tokens_index: HashMap<TokenId, usize>,
    /// Tokens by owner
    owned_tokens: HashMap<Address, Vec<TokenId>>,
    /// Token index in owned_tokens
    owned_tokens_index: HashMap<TokenId, usize>,
}

impl EnumerableNFT {
    pub fn new() -> Self {
        Self {
            all_tokens: Vec::new(),
            all_tokens_index: HashMap::new(),
            owned_tokens: HashMap::new(),
            owned_tokens_index: HashMap::new(),
        }
    }

    /// Add token to enumeration
    pub fn add_token(&mut self, token_id: TokenId, owner: Address) {
        // Add to all tokens
        self.all_tokens_index.insert(token_id, self.all_tokens.len());
        self.all_tokens.push(token_id);

        // Add to owner's tokens
        let owner_tokens = self.owned_tokens.entry(owner).or_insert_with(Vec::new);
        self.owned_tokens_index.insert(token_id, owner_tokens.len());
        owner_tokens.push(token_id);
    }

    /// Remove token from enumeration
    pub fn remove_token(&mut self, token_id: TokenId, owner: &Address) {
        // Remove from all tokens
        if let Some(&index) = self.all_tokens_index.get(&token_id) {
            let last_token = *self.all_tokens.last().unwrap();
            self.all_tokens.swap_remove(index);
            self.all_tokens_index.remove(&token_id);
            if last_token != token_id {
                self.all_tokens_index.insert(last_token, index);
            }
        }

        // Remove from owner's tokens
        if let Some(owner_tokens) = self.owned_tokens.get_mut(owner) {
            if let Some(&index) = self.owned_tokens_index.get(&token_id) {
                let last_token = *owner_tokens.last().unwrap();
                owner_tokens.swap_remove(index);
                self.owned_tokens_index.remove(&token_id);
                if last_token != token_id {
                    self.owned_tokens_index.insert(last_token, index);
                }
            }
        }
    }

    /// Get token by index
    pub fn token_by_index(&self, index: usize) -> Option<TokenId> {
        self.all_tokens.get(index).copied()
    }

    /// Get token of owner by index
    pub fn token_of_owner_by_index(&self, owner: &Address, index: usize) -> Option<TokenId> {
        self.owned_tokens
            .get(owner)
            .and_then(|tokens| tokens.get(index).copied())
    }

    /// Total supply
    pub fn total_supply(&self) -> usize {
        self.all_tokens.len()
    }
}

impl Default for EnumerableNFT {
    fn default() -> Self {
        Self::new()
    }
}
