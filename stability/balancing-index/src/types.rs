use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Represents an asset in the basket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Asset {
    /// US Dollar
    USD,
    /// Bitcoin
    BTC,
    /// Ethereum
    ETH,
    /// Gold
    Gold,
    /// S&P 500 Index
    SP500,
    /// Custom asset with identifier
    Custom(String),
}

impl Asset {
    /// Get the human-readable name of the asset
    pub fn name(&self) -> &str {
        match self {
            Asset::USD => "US Dollar",
            Asset::BTC => "Bitcoin",
            Asset::ETH => "Ethereum",
            Asset::Gold => "Gold",
            Asset::SP500 => "S&P 500",
            Asset::Custom(name) => name,
        }
    }

    /// Get the ticker symbol of the asset
    pub fn ticker(&self) -> &str {
        match self {
            Asset::USD => "USD",
            Asset::BTC => "BTC",
            Asset::ETH => "ETH",
            Asset::Gold => "XAU",
            Asset::SP500 => "SPX",
            Asset::Custom(name) => name,
        }
    }
}

/// Weight allocation for an asset in the basket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetWeight {
    /// The asset
    pub asset: Asset,
    /// Weight in basis points (10000 = 100%)
    pub weight_bps: u16,
    /// Minimum weight allowed (in basis points)
    pub min_weight_bps: u16,
    /// Maximum weight allowed (in basis points)
    pub max_weight_bps: u16,
}

impl AssetWeight {
    /// Create a new asset weight
    pub fn new(asset: Asset, weight_bps: u16) -> Self {
        Self {
            asset,
            weight_bps,
            min_weight_bps: weight_bps.saturating_sub(500), // Allow 5% variation
            max_weight_bps: weight_bps.saturating_add(500).min(10000),
        }
    }

    /// Create with custom min/max bounds
    pub fn with_bounds(
        asset: Asset,
        weight_bps: u16,
        min_weight_bps: u16,
        max_weight_bps: u16,
    ) -> Self {
        Self {
            asset,
            weight_bps,
            min_weight_bps,
            max_weight_bps,
        }
    }

    /// Check if a weight is within acceptable bounds
    pub fn is_within_bounds(&self, weight_bps: u16) -> bool {
        weight_bps >= self.min_weight_bps && weight_bps <= self.max_weight_bps
    }

    /// Get the weight as a decimal (0.0 to 1.0)
    pub fn weight_decimal(&self) -> f64 {
        self.weight_bps as f64 / 10000.0
    }
}

/// Composition of the index basket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexComposition {
    /// Asset weights in the basket
    pub weights: Vec<AssetWeight>,
    /// Timestamp of this composition
    pub timestamp: DateTime<Utc>,
    /// Version number for tracking changes
    pub version: u64,
}

impl IndexComposition {
    /// Create a new index composition
    pub fn new(weights: Vec<AssetWeight>) -> Result<Self> {
        let composition = Self {
            weights,
            timestamp: Utc::now(),
            version: 1,
        };
        composition.validate()?;
        Ok(composition)
    }

    /// Validate that weights sum to 100%
    pub fn validate(&self) -> Result<()> {
        let total: u32 = self.weights.iter().map(|w| w.weight_bps as u32).sum();

        if total != 10000 {
            return Err(BalancingError::InvalidWeights {
                total_bps: total,
                expected_bps: 10000,
            });
        }

        // Check for duplicate assets
        let mut seen = std::collections::HashSet::new();
        for weight in &self.weights {
            if !seen.insert(&weight.asset) {
                return Err(BalancingError::DuplicateAsset(weight.asset.ticker().to_string()));
            }
        }

        Ok(())
    }

    /// Get the weight for a specific asset
    pub fn get_weight(&self, asset: &Asset) -> Option<u16> {
        self.weights
            .iter()
            .find(|w| &w.asset == asset)
            .map(|w| w.weight_bps)
    }

    /// Update to a new version with new weights
    pub fn update(&mut self, new_weights: Vec<AssetWeight>) -> Result<()> {
        let new_composition = Self {
            weights: new_weights,
            timestamp: Utc::now(),
            version: self.version + 1,
        };
        new_composition.validate()?;
        *self = new_composition;
        Ok(())
    }

    /// Get the default composition (40% USD, 25% BTC, 20% ETH, 10% Gold, 5% SP500)
    pub fn default_composition() -> Self {
        Self {
            weights: vec![
                AssetWeight::new(Asset::USD, 4000),   // 40%
                AssetWeight::new(Asset::BTC, 2500),   // 25%
                AssetWeight::new(Asset::ETH, 2000),   // 20%
                AssetWeight::new(Asset::Gold, 1000),  // 10%
                AssetWeight::new(Asset::SP500, 500),  // 5%
            ],
            timestamp: Utc::now(),
            version: 1,
        }
    }
}

/// Errors related to balancing index operations
#[derive(Error, Debug)]
pub enum BalancingError {
    #[error("Invalid weights: total {total_bps}bps, expected {expected_bps}bps")]
    InvalidWeights {
        total_bps: u32,
        expected_bps: u32,
    },

    #[error("Duplicate asset in composition: {0}")]
    DuplicateAsset(String),

    #[error("Asset not found in basket: {0}")]
    AssetNotFound(String),

    #[error("Price not available for asset: {0}")]
    PriceNotAvailable(String),

    #[error("Weight out of bounds: {current_bps}bps not in [{min_bps}, {max_bps}]")]
    WeightOutOfBounds {
        current_bps: u16,
        min_bps: u16,
        max_bps: u16,
    },

    #[error("Rebalancing frequency violation: last rebalance {last_rebalance_hours}h ago, minimum {min_hours}h")]
    RebalancingFrequency {
        last_rebalance_hours: i64,
        min_hours: i64,
    },

    #[error("Calculation error: {0}")]
    CalculationError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

pub type Result<T> = std::result::Result<T, BalancingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_names() {
        assert_eq!(Asset::USD.name(), "US Dollar");
        assert_eq!(Asset::BTC.ticker(), "BTC");
        assert_eq!(Asset::ETH.name(), "Ethereum");
    }

    #[test]
    fn test_asset_weight_creation() {
        let weight = AssetWeight::new(Asset::USD, 4000);
        assert_eq!(weight.weight_bps, 4000);
        assert_eq!(weight.weight_decimal(), 0.4);
    }

    #[test]
    fn test_asset_weight_bounds() {
        let weight = AssetWeight::new(Asset::USD, 4000);
        assert!(weight.is_within_bounds(3800)); // Within -2%
        assert!(weight.is_within_bounds(4200)); // Within +2%
        assert!(!weight.is_within_bounds(3000)); // Too low
        assert!(!weight.is_within_bounds(5000)); // Too high
    }

    #[test]
    fn test_default_composition() {
        let composition = IndexComposition::default_composition();
        assert!(composition.validate().is_ok());
        assert_eq!(composition.weights.len(), 5);

        let total: u32 = composition.weights.iter().map(|w| w.weight_bps as u32).sum();
        assert_eq!(total, 10000); // 100%
    }

    #[test]
    fn test_composition_validation() {
        let valid_weights = vec![
            AssetWeight::new(Asset::USD, 5000),
            AssetWeight::new(Asset::BTC, 5000),
        ];
        assert!(IndexComposition::new(valid_weights).is_ok());

        let invalid_weights = vec![
            AssetWeight::new(Asset::USD, 5000),
            AssetWeight::new(Asset::BTC, 4000), // Only 90%
        ];
        assert!(IndexComposition::new(invalid_weights).is_err());
    }

    #[test]
    fn test_duplicate_asset_detection() {
        let duplicate_weights = vec![
            AssetWeight::new(Asset::USD, 5000),
            AssetWeight::new(Asset::USD, 5000), // Duplicate!
        ];
        assert!(IndexComposition::new(duplicate_weights).is_err());
    }

    #[test]
    fn test_get_weight() {
        let composition = IndexComposition::default_composition();
        assert_eq!(composition.get_weight(&Asset::USD), Some(4000));
        assert_eq!(composition.get_weight(&Asset::BTC), Some(2500));
        assert_eq!(composition.get_weight(&Asset::Custom("UNKNOWN".to_string())), None);
    }

    #[test]
    fn test_composition_update() {
        let mut composition = IndexComposition::default_composition();
        let initial_version = composition.version;

        let new_weights = vec![
            AssetWeight::new(Asset::USD, 6000), // Increase USD
            AssetWeight::new(Asset::BTC, 4000), // Decrease BTC
        ];

        assert!(composition.update(new_weights).is_ok());
        assert_eq!(composition.version, initial_version + 1);
        assert_eq!(composition.get_weight(&Asset::USD), Some(6000));
    }
}
