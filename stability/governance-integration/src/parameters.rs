use crate::types::{GovernanceError, Result};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Manages stability system parameters via governance
pub struct ParameterController {
    parameters: HashMap<String, ParameterValue>,
    change_history: Vec<ParameterChange>,
}

#[derive(Debug, Clone)]
struct ParameterValue {
    value: String,
    min: Option<String>,
    max: Option<String>,
    last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct ParameterChange {
    name: String,
    old_value: String,
    new_value: String,
    timestamp: DateTime<Utc>,
    proposer: String,
}

impl ParameterController {
    pub fn new() -> Self {
        let mut controller = Self {
            parameters: HashMap::new(),
            change_history: Vec::new(),
        };
        controller.initialize_default_parameters();
        controller
    }

    pub fn get_parameter(&self, name: &str) -> Option<String> {
        self.parameters.get(name).map(|p| p.value.clone())
    }

    pub fn update_parameter(&mut self, name: String, value: String, proposer: String) -> Result<()> {
        let old_value = self.get_parameter(&name).unwrap_or_default();

        // Validate parameter exists
        if !self.parameters.contains_key(&name) {
            return Err(GovernanceError::InvalidParameter(format!("Unknown parameter: {}", name)));
        }

        // Validate bounds if they exist
        if let Some(param) = self.parameters.get(&name) {
            if let (Some(min), Some(max)) = (&param.min, &param.max) {
                // For numeric parameters, validate range
                if let (Ok(val), Ok(min_val), Ok(max_val)) = (
                    value.parse::<f64>(),
                    min.parse::<f64>(),
                    max.parse::<f64>()
                ) {
                    if val < min_val || val > max_val {
                        return Err(GovernanceError::InvalidParameter(
                            format!("Value {} outside bounds [{}, {}]", val, min_val, max_val)
                        ));
                    }
                }
            }
        }

        // Update parameter
        self.parameters.insert(name.clone(), ParameterValue {
            value: value.clone(),
            min: self.parameters.get(&name).and_then(|p| p.min.clone()),
            max: self.parameters.get(&name).and_then(|p| p.max.clone()),
            last_updated: Utc::now(),
        });

        // Record change
        self.change_history.push(ParameterChange {
            name,
            old_value,
            new_value: value,
            timestamp: Utc::now(),
            proposer,
        });

        Ok(())
    }

    pub fn get_change_history(&self, name: &str) -> Vec<&ParameterChange> {
        self.change_history.iter().filter(|c| c.name == name).collect()
    }

    fn initialize_default_parameters(&mut self) {
        let defaults = vec![
            ("max_daily_adjustment_bps", "500", Some("0"), Some("1000")),
            ("min_deviation_bps", "500", Some("100"), Some("2000")),
            ("target_price_update_frequency", "300", Some("60"), Some("3600")),
            ("circuit_breaker_threshold_yellow", "1000", Some("500"), Some("2000")),
            ("circuit_breaker_threshold_red", "2000", Some("1000"), Some("5000")),
            ("treasury_max_daily_spend_bps", "100", Some("10"), Some("500")),
            ("liquidity_min_apr_bps", "500", Some("100"), Some("5000")),
        ];

        for (name, value, min, max) in defaults {
            self.parameters.insert(name.to_string(), ParameterValue {
                value: value.to_string(),
                min: min.map(|s| s.to_string()),
                max: max.map(|s| s.to_string()),
                last_updated: Utc::now(),
            });
        }
    }
}

impl Default for ParameterController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_controller_creation() {
        let controller = ParameterController::new();
        assert!(controller.get_parameter("max_daily_adjustment_bps").is_some());
    }

    #[test]
    fn test_get_parameter() {
        let controller = ParameterController::new();
        let value = controller.get_parameter("max_daily_adjustment_bps").unwrap();
        assert_eq!(value, "500");
    }

    #[test]
    fn test_update_parameter() {
        let mut controller = ParameterController::new();
        let result = controller.update_parameter(
            "max_daily_adjustment_bps".to_string(),
            "600".to_string(),
            "proposer1".to_string()
        );
        assert!(result.is_ok());
        assert_eq!(controller.get_parameter("max_daily_adjustment_bps").unwrap(), "600");
    }

    #[test]
    fn test_update_invalid_parameter() {
        let mut controller = ParameterController::new();
        let result = controller.update_parameter(
            "nonexistent_param".to_string(),
            "100".to_string(),
            "proposer1".to_string()
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_parameter_bounds() {
        let mut controller = ParameterController::new();

        // Try to set value above max (1000)
        let result = controller.update_parameter(
            "max_daily_adjustment_bps".to_string(),
            "1500".to_string(),
            "proposer1".to_string()
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_change_history() {
        let mut controller = ParameterController::new();

        controller.update_parameter(
            "max_daily_adjustment_bps".to_string(),
            "600".to_string(),
            "proposer1".to_string()
        ).unwrap();

        controller.update_parameter(
            "max_daily_adjustment_bps".to_string(),
            "700".to_string(),
            "proposer2".to_string()
        ).unwrap();

        let history = controller.get_change_history("max_daily_adjustment_bps");
        assert_eq!(history.len(), 2);
    }
}
