//! Strategy Event Primitives
//!
//! Visual primitives for strategy events like crossovers, breakdowns,
//! divergences, pattern matches, and custom signals.
//!
//! These are non-interactive markers that represent strategy-generated events.
//! Unlike user-drawn primitives, events cannot be dragged or edited.
//!
//! # Event Types
//!
//! - **Crossover** - MA crossover, MACD line cross, etc.
//! - **Breakdown** - Level breakout/breakdown events
//! - **Divergence** - RSI/MACD divergence markers
//! - **PatternMatch** - Detected pattern (triangle, H&S, etc.)
//! - **ZoneEvent** - Supply/demand zone, order block events
//! - **VolumeEvent** - Volume spike, climax, dry-up events
//! - **TrendEvent** - Trend change, reversal, continuation
//! - **MomentumEvent** - Momentum shift, exhaustion
//! - **CustomEvent** - User-defined strategy events

mod breakdown;
mod crossover;
mod custom_event;
mod divergence;
mod momentum_event;
mod pattern_match;
mod trend_event;
mod volume_event;
mod zone_event;

pub use breakdown::{metadata as breakdown_metadata, Breakdown, BreakdownType};
pub use crossover::{metadata as crossover_metadata, Crossover, CrossoverDirection, CrossoverType};
pub use custom_event::{metadata as custom_event_metadata, CustomEvent, CustomEventStyle};
pub use divergence::{metadata as divergence_metadata, Divergence, DivergenceType};
pub use momentum_event::{metadata as momentum_event_metadata, MomentumEvent, MomentumEventType};
pub use pattern_match::{metadata as pattern_match_metadata, PatternMatch, PatternType};
pub use trend_event::{metadata as trend_event_metadata, TrendEvent, TrendEventType};
pub use volume_event::{metadata as volume_event_metadata, VolumeEvent, VolumeEventType};
pub use zone_event::{metadata as zone_event_metadata, ZoneAction, ZoneEvent, ZoneType};
