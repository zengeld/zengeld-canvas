//! Layout System
//!
//! Chart layout and organization:
//! - `pane` - Sub-pane system for indicator panels
//! - `multichart` - Multichart grid layouts (2x2, 1+3, etc.)

pub mod multichart;
pub mod pane;

// Re-exports - Pane system
pub use pane::{coordinate_utils, Pane, PaneGeometry, PaneId, PaneManager, SubPane, MAIN_PANE};

// Re-exports - Multichart
pub use multichart::{CellBounds, CellId, LayoutCell, MultichartLayout};
