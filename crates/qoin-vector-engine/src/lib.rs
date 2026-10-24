//! Deterministic, research-stage market-force vector decision engine.
//!
//! This crate emits signals only. It does not size positions, place orders, or
//! claim profitability. Inputs must be causal (known at decision time) and
//! normalized using past-only statistics by the caller.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ForceObservation {
    pub ofi: f64,
    pub liquidity: f64,
    pub volatility: f64,
    pub funding: f64,
    pub open_interest_change: f64,
    pub liquidation_pressure: f64,
    pub basis: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Decision {
    Buy,
    Sell,
    Close,
    Wait,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Minimum absolute directional score to consider a trade.
    pub signal_threshold: f64,
    /// Maximum allowed absolute input after normalization/clipping.
    pub input_clip: f64,
    /// Maximum absolute output score.
    pub score_clip: f64,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self { signal_threshold: 0.65, input_clip: 3.0, score_clip: 1.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Signal {
    pub decision: Decision,
    pub score: f64,
    pub config_version: String,
    pub rationale: String,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EngineError {
    #[error("all observations must be finite")]
    NonFiniteInput,
    #[error("invalid engine configuration")]
    InvalidConfig,
}

pub fn decide(
    input: ForceObservation,
    config: &EngineConfig,
    config_version: &str,
    has_open_position: bool,
) -> Result<Signal, EngineError> {
    let values = [
        input.ofi, input.liquidity, input.volatility, input.funding,
        input.open_interest_change, input.liquidation_pressure, input.basis,
    ];
    if values.iter().any(|v| !v.is_finite()) {
        return Err(EngineError::NonFiniteInput);
    }
    if !config.signal_threshold.is_finite()
        || !config.input_clip.is_finite()
        || !config.score_clip.is_finite()
        || config.signal_threshold <= 0.0
        || config.input_clip <= 0.0
        || config.score_clip <= 0.0
        || config.score_clip > 1.0
    {
        return Err(EngineError::InvalidConfig);
    }

    let clip = |v: f64| v.clamp(-config.input_clip, config.input_clip);
    // Explicit baseline coefficients; not trained and not evidence of edge.
    // Volatility is a risk/context force, so it dampens direction rather than
    // being interpreted as bullish or bearish on its own.
    let directional = 0.30 * clip(input.ofi)
        + 0.18 * clip(input.liquidity)
        + 0.12 * clip(input.open_interest_change)
        + 0.12 * clip(input.liquidation_pressure)
        - 0.10 * clip(input.funding)
        - 0.08 * clip(input.basis);
    let dampener = 1.0 / (1.0 + clip(input.volatility).abs() * 0.15);
    let score = (directional * dampener).clamp(-config.score_clip, config.score_clip);
    let decision = if score >= config.signal_threshold {
        Decision::Buy
    } else if score <= -config.signal_threshold {
        Decision::Sell
    } else if has_open_position {
        Decision::Close
    } else {
        Decision::Wait
    };
    Ok(Signal {
        decision,
        score,
        config_version: config_version.to_owned(),
        rationale: "research baseline; signal only; risk gate and execution are separate".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ForceObservation {
        ForceObservation {
            ofi: 2.0, liquidity: 1.0, volatility: 0.2, funding: -0.5,
            open_interest_change: 1.0, liquidation_pressure: 0.5, basis: 0.0,
        }
    }

    #[test]
    fn strong_positive_force_produces_buy() {
        assert_eq!(decide(sample(), &EngineConfig::default(), "baseline-v1", false).unwrap().decision, Decision::Buy);
    }

    #[test]
    fn weak_signal_without_position_waits() {
        let input = ForceObservation { ofi: 0.0, liquidity: 0.0, volatility: 0.0, funding: 0.0, open_interest_change: 0.0, liquidation_pressure: 0.0, basis: 0.0 };
        assert_eq!(decide(input, &EngineConfig::default(), "baseline-v1", false).unwrap().decision, Decision::Wait);
    }

    #[test]
    fn non_finite_values_are_rejected() {
        let mut input = sample();
        input.ofi = f64::NAN;
        assert_eq!(decide(input, &EngineConfig::default(), "baseline-v1", false).unwrap_err(), EngineError::NonFiniteInput);
    }
}
