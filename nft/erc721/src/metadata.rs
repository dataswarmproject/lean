//! NFT metadata structures

use serde::{Deserialize, Serialize};

/// Token metadata URI
pub type MetadataUri = String;

/// NFT attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAttribute {
    pub trait_type: String,
    pub value: String,
}

/// Token metadata (ERC-721 metadata standard)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    #[serde(default)]
    pub attributes: Vec<TokenAttribute>,
}

impl TokenMetadata {
    pub fn new(name: String, description: String, image: String) -> Self {
        Self {
            name,
            description,
            image,
            attributes: vec![],
        }
    }

    pub fn add_attribute(&mut self, trait_type: String, value: String) {
        self.attributes.push(TokenAttribute { trait_type, value });
    }
}
