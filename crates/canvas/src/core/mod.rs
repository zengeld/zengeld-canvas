//! Core types and utilities
//!
//! Foundational types shared across the entire crate:
//! - `Bar` - OHLCV data structure
//! - `Theme` - Color palette for dark/light themes
//! - `ChartConfig` - Global configuration system
//! - Layout constants (scale dimensions, toolbar sizes)
//! - Utility functions (crisp rendering, color parsing)

mod color;
pub mod config;
mod format;
mod math;
mod types;

// Re-export types
pub use types::{
    crisp,
    crisp_rect,
    Bar,
    Theme,
    BOTTOM_SIDEBAR_HEIGHT,
    BOTTOM_TOOLBAR_HEIGHT,
    // Sidebar & toolbar constants
    LEFT_SIDEBAR_WIDTH,
    LEFT_TOOLBAR_WIDTH,
    // Price scale constants
    PRICE_SCALE_BORDER_SIZE,
    PRICE_SCALE_FONT,
    PRICE_SCALE_FONT_SIZE,
    PRICE_SCALE_FONT_SIZE_MAX,
    PRICE_SCALE_FONT_SIZE_MIN,
    PRICE_SCALE_LABEL_OFFSET,
    PRICE_SCALE_MIN_WIDTH,
    PRICE_SCALE_PADDING_INNER,
    PRICE_SCALE_PADDING_OUTER,
    PRICE_SCALE_TICK_LENGTH,
    PRICE_SCALE_WIDTH,
    RIGHT_SIDEBAR_WIDTH,
    RIGHT_TOOLBAR_WIDTH,
    STATUS_BAR_HEIGHT,
    TIME_SCALE_FONT_SIZE,
    TIME_SCALE_HEIGHT,
    TOP_TOOLBAR_HEIGHT,
};

// Re-export utility functions
pub use color::parse_css_color;
pub use format::format_indicator_value;
pub use math::catmull_rom_spline;

// Re-export configuration system
pub use config::{
    AreaConfig,
    // Series configs
    CandlestickConfig,
    ChartConfig,
    // Color
    Color as ConfigColor,
    CrosshairConfig,
    FontConfig,
    FontWeight,
    // Overlay configs
    GridConfig,
    HistogramConfig,
    LegendConfig,
    LineConfig,
    LineStyleType,
    // Scale configs
    PriceScaleConfig,
    // Primitive config
    PrimitiveConfig,
    TimeScaleConfig,
    WatermarkConfig,
};
