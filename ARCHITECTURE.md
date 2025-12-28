# zengeld-canvas Architecture

This document describes the internal architecture of zengeld-canvas, a high-performance financial charting library.

## Table of Contents

- [Overview](#overview)
- [Module Diagram](#module-diagram)
- [Rendering Pipeline](#rendering-pipeline)
- [Coordinate Transformation System](#coordinate-transformation-system)
- [Primitive Registry Pattern](#primitive-registry-pattern)
- [Indicator System](#indicator-system)
- [Theme System](#theme-system)
- [Layout System](#layout-system)
- [Bindings Architecture](#bindings-architecture)

---

## Overview

zengeld-canvas is a Rust library for rendering financial charts to SVG. Key design principles:

1. **Headless Rendering**: Pure SVG output, no UI framework dependencies
2. **Separation of Concerns**: Data computation happens externally; library only renders
3. **Platform Agnostic**: Core library works everywhere Rust runs
4. **Binding Friendly**: Clean API designed for Python/WASM wrappers

```
┌─────────────────────────────────────────────────────────────────┐
│                      User Code (Python/JS/Rust)                  │
├─────────────────────────────────────────────────────────────────┤
│  canvas-py (PyO3)  │  canvas-wasm (wasm-bindgen)  │  Direct Rust │
├─────────────────────────────────────────────────────────────────┤
│                         canvas (core library)                    │
│  ┌─────────┐ ┌──────────┐ ┌───────────┐ ┌────────┐ ┌──────────┐ │
│  │   API   │ │  Coords  │ │ Primitives│ │ Render │ │  Model   │ │
│  │ (Chart) │ │(Viewport)│ │ (90+types)│ │  (SVG) │ │(Series/  │ │
│  └─────────┘ └──────────┘ └───────────┘ └────────┘ │Indicators)│ │
│                                                     └──────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## Module Diagram

```
zengeld_canvas/
│
├── api/                         # High-level API
│   ├── chart.rs                 # Chart builder, ChartRenderer
│   └── config.rs                # Configuration structs
│
├── coords/                      # Coordinate Systems
│   ├── viewport.rs              # Viewport (main API)
│   ├── time_scale.rs            # TimeScale (X-axis)
│   └── price_scale.rs           # PriceScale (Y-axis)
│
├── core/                        # Core Types
│   ├── types.rs                 # Bar, Theme, constants
│   └── config.rs                # Color, Font configs
│
├── model/                       # Data Models
│   ├── series/                  # Series types
│   │   ├── candlestick.rs
│   │   ├── line.rs
│   │   └── ...
│   ├── indicators.rs            # Indicator presets (60+)
│   ├── overlays/                # Overlay types
│   └── annotations/             # Text, markers
│
├── primitives/                  # Drawing Primitives
│   ├── mod.rs                   # Primitive trait
│   ├── registry.rs              # PrimitiveRegistry
│   ├── core/
│   │   ├── render.rs            # RenderContext trait
│   │   └── types.rs             # PrimitiveData, colors
│   └── catalog/                 # 90+ primitive types
│       ├── lines/               # 9 line types
│       ├── channels/            # 4 channel types
│       ├── shapes/              # 10 shape types
│       ├── fibonacci/           # 11 fib tools
│       ├── gann/                # 4 gann tools
│       ├── pitchforks/          # 4 pitchfork types
│       ├── patterns/            # 6 patterns
│       ├── elliott/             # 5 elliott waves
│       ├── arrows/              # 4 arrow types
│       ├── annotations/         # 11 annotation types
│       ├── cycles/              # 3 cycle tools
│       ├── projection/          # 6 projection tools
│       ├── volume/              # 3 volume tools
│       ├── measurement/         # 3 measurement tools
│       ├── brushes/             # 2 brush types
│       ├── icons/               # 2 icon types
│       ├── events/              # 9 event markers
│       ├── signals/             # Trading signals
│       ├── trades/              # Trade visualization
│       └── utils/               # Helpers
│
├── render/                      # Rendering Engine
│   ├── engine/
│   │   ├── backend.rs           # RenderBackend trait
│   │   ├── svg_backend.rs       # SVG output
│   │   ├── path.rs              # Path commands
│   │   └── types.rs             # Color, Point, Rect
│   └── chart/
│       ├── candlesticks.rs      # Candlestick rendering
│       ├── line.rs              # Line rendering
│       └── ...
│
├── layout/                      # Layout System
│   ├── multichart.rs            # Grid layouts
│   └── panes.rs                 # Subpane layout
│
└── theme/                       # Theme System
    └── ui_theme.rs              # UITheme, RuntimeTheme
```

### Dependency Flow

```
                    ┌─────────────┐
                    │    api/     │
                    │   chart.rs  │
                    └──────┬──────┘
                           │ uses
           ┌───────────────┼───────────────┐
           │               │               │
           ▼               ▼               ▼
    ┌────────────┐  ┌────────────┐  ┌────────────┐
    │   coords/  │  │   model/   │  │  render/   │
    │  viewport  │  │ indicators │  │  engine/   │
    └────────────┘  │   series   │  │  chart/    │
           │        └─────┬──────┘  └─────┬──────┘
           │              │               │
           │              ▼               │
           │       ┌────────────┐         │
           └──────►│ primitives/│◄────────┘
                   │  registry  │
                   │   catalog  │
                   └─────┬──────┘
                         │
                         ▼
                   ┌────────────┐
                   │   core/    │
                   │   types    │
                   └────────────┘
```

---

## Rendering Pipeline

The rendering pipeline transforms data into SVG through these stages:

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│  ChartConfig │───►│   Viewport   │───►│  RenderOps   │───►│     SVG      │
│  + Bar Data  │    │  Calculation │    │  Generation  │    │    Output    │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
```

### Detailed Pipeline

```
1. INPUT
   ├── ChartConfig (width, height, theme, etc.)
   ├── Bar[] (OHLCV data)
   ├── Indicator[] (computed indicator values)
   ├── Primitive[] (drawing primitives)
   └── Signal[] (trading signals)

2. COORDINATE SETUP
   ├── Calculate visible bar range
   ├── Auto-scale price range from visible data
   ├── Setup TimeScale (bar index → pixel X)
   └── Setup PriceScale (price → pixel Y)

3. LAYOUT CALCULATION
   ├── Main chart area dimensions
   ├── Subpane heights (for RSI, MACD, etc.)
   └── Scale areas (price scale, time scale)

4. RENDERING (to RenderBatch)
   ├── Background and grid
   ├── Main series (candlesticks/line/area)
   ├── Overlay indicators
   ├── Primitives (drawings)
   ├── Signals (buy/sell markers)
   ├── Subpane indicators
   └── Scales (price labels, time labels)

5. OUTPUT
   └── SvgBackend.to_svg() → String
```

### Code Path

```rust
// Entry point: crates/canvas/src/api/chart.rs

impl ChartRenderer {
    pub fn render_svg(&self) -> String {
        // 1. Setup
        let backend = SvgBackend::new(width, height, dpr);

        // 2. Coordinate calculation
        let (price_min, price_max) = self.price_range(&overlays);
        let bar_to_x = |i: usize| -> f64 { bar_spacing * (i as f64 + 0.5) };
        let price_to_y = |price: f64| -> f64 { ... };

        // 3. Render grid
        self.draw_grid(&mut backend, ...);

        // 4. Render series
        self.render_main_series(&mut batch, &bar_to_x, &price_to_y, ...);

        // 5. Render indicators
        self.render_overlay_indicators(&mut backend, ...);
        self.render_subpane_indicator(&mut backend, ...);

        // 6. Render primitives
        self.render_primitives(&mut backend, &bar_to_x, &price_to_y, ...);

        // 7. Render scales
        self.render_price_scale(&mut backend, ...);
        self.render_time_scale(&mut backend, ...);

        // 8. Output
        backend.to_svg()
    }
}
```

---

## Coordinate Transformation System

The coordinate system maps between three spaces:

```
DATA SPACE              CHART SPACE              SCREEN SPACE
(bar index, price)      (normalized 0-1)         (pixels)

     │                       │                       │
Bar 0│────────────►   0.0 ───┼───────────►   0 ──────┤
     │                       │                       │
Bar N│────────────►   1.0 ───┼───────────►  Width ───┤
     │                       │                       │
Price Max─────────►   0.0 ───┼───────────►   0 ──────┤ (Y is inverted)
     │                       │                       │
Price Min─────────►   1.0 ───┼───────────►  Height ──┤
```

### Viewport

`Viewport` is the main coordinate system API combining TimeScale and PriceScale:

```rust
// crates/canvas/src/coords/viewport.rs

pub struct Viewport {
    pub time_scale: TimeScale,   // X-axis
    pub price_scale: PriceScale, // Y-axis
    pub chart_height: f64,
}

impl Viewport {
    // X-axis: Bar index → Pixel
    pub fn bar_to_x(&self, bar_idx: usize) -> f64 {
        self.time_scale.bar_to_x(bar_idx)
    }

    // Y-axis: Price → Pixel (inverted)
    pub fn price_to_y(&self, price: f64) -> f64 {
        self.price_scale.price_to_y(price, self.chart_height)
    }

    // Navigation
    pub fn zoom(&mut self, factor: f64, anchor_x: f64);
    pub fn pan(&mut self, bar_delta: f64);
    pub fn scroll_to_end(&mut self);
}
```

### TimeScale (X-Axis)

```rust
// crates/canvas/src/coords/time_scale.rs

pub struct TimeScale {
    pub chart_width: f64,     // Available width in pixels
    pub bar_count: usize,     // Total number of bars
    pub bar_spacing: f64,     // Pixels per bar (zoom level)
    pub view_start: f64,      // First visible bar (can be fractional)
}

impl TimeScale {
    /// Bar index to X pixel (center of bar)
    pub fn bar_to_x(&self, bar_idx: usize) -> f64 {
        let offset = bar_idx as f64 - self.view_start;
        offset * self.bar_spacing + self.bar_spacing / 2.0
    }

    /// X pixel to bar index
    pub fn x_to_bar(&self, x: f64) -> Option<usize> {
        let bar_f = self.view_start + x / self.bar_spacing;
        if bar_f >= 0.0 && (bar_f as usize) < self.bar_count {
            Some(bar_f as usize)
        } else {
            None
        }
    }
}
```

### PriceScale (Y-Axis)

```rust
// crates/canvas/src/coords/price_scale.rs

pub enum PriceScaleMode {
    Normal,      // Linear absolute prices
    Percent,     // Percentage from base price
    Logarithmic, // Log scale
}

pub struct PriceScale {
    pub price_min: f64,
    pub price_max: f64,
    pub mode: PriceScaleMode,
    pub auto_scale: bool,
}

impl PriceScale {
    /// Price to Y pixel (inverted - high prices at top)
    pub fn price_to_y(&self, price: f64, chart_height: f64) -> f64 {
        match self.mode {
            PriceScaleMode::Normal => {
                let range = self.price_max - self.price_min;
                chart_height * (1.0 - (price - self.price_min) / range)
            }
            PriceScaleMode::Logarithmic => {
                let log_price = price.ln();
                let log_min = self.price_min.ln();
                let log_max = self.price_max.ln();
                chart_height * (1.0 - (log_price - log_min) / (log_max - log_min))
            }
            // ...
        }
    }
}
```

### Nice Number Algorithm

The price scale uses a "nice number" algorithm for tick placement:

```rust
/// Multiplier pattern for professional-looking ticks: [2, 2.5, 2]
/// Produces: 1, 2, 5, 10, 20, 50, 100, 200, 500, ...
pub const NICE_MULTIPLIERS: [f64; 3] = [2.0, 2.5, 2.0];

pub fn nice_number(value: f64) -> f64 {
    let exp = value.log10().floor();
    let base = 10.0_f64.powf(exp);

    let mut current = base;
    let mut idx = 0;
    while current < value {
        current *= NICE_MULTIPLIERS[idx % 3];
        idx += 1;
    }
    current
}
```

---

## Primitive Registry Pattern

The registry pattern allows adding new primitives without modifying existing code.

```
┌─────────────────────────────────────────────────────────────────┐
│                      PrimitiveRegistry                           │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  HashMap<&'static str, PrimitiveMetadata>                  │ │
│  │                                                             │ │
│  │  "trend_line"    → { factory: create_trend_line, ... }     │ │
│  │  "fib_retracement" → { factory: create_fib, ... }          │ │
│  │  "rectangle"     → { factory: create_rect, ... }           │ │
│  │  ...                                                        │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼ create()
                    ┌─────────────────────┐
                    │  Box<dyn Primitive> │
                    └─────────────────────┘
```

### Registry Structure

```rust
// crates/canvas/src/primitives/registry.rs

pub type PrimitiveFactory = fn(points: &[(f64, f64)], color: &str) -> Box<dyn Primitive>;

pub struct PrimitiveMetadata {
    pub type_id: &'static str,        // Unique ID: "trend_line"
    pub display_name: &'static str,   // UI name: "Trend Line"
    pub kind: PrimitiveKind,          // Category: Line, Shape, etc.
    pub factory: PrimitiveFactory,    // Factory function
    pub supports_text: bool,          // Can have text label
    pub has_levels: bool,             // Has configurable levels (Fib, Gann)
    pub has_points_config: bool,      // Has configurable points (Elliott)
}

pub struct PrimitiveRegistry {
    primitives: HashMap<&'static str, PrimitiveMetadata>,
    by_kind: HashMap<PrimitiveKind, Vec<&'static str>>,
}

impl PrimitiveRegistry {
    /// Get global singleton
    pub fn global() -> &'static RwLock<PrimitiveRegistry> {
        static REGISTRY: OnceLock<RwLock<PrimitiveRegistry>> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let mut registry = PrimitiveRegistry::new();
            registry.register_builtins();
            RwLock::new(registry)
        })
    }

    /// Create primitive by type ID
    pub fn create(&self, type_id: &str, points: &[(f64, f64)], color: Option<&str>)
        -> Option<Box<dyn Primitive>>
    {
        let meta = self.primitives.get(type_id)?;
        Some((meta.factory)(points, color.unwrap_or("#2196F3")))
    }
}
```

### Primitive Trait

```rust
// crates/canvas/src/primitives/mod.rs

pub trait Primitive: Send + Sync {
    /// Unique type identifier
    fn type_id(&self) -> &'static str;

    /// Display name for UI
    fn display_name(&self) -> &str;

    /// Category (Line, Shape, Fibonacci, etc.)
    fn kind(&self) -> PrimitiveKind;

    /// Common data (color, width, style, text)
    fn data(&self) -> &PrimitiveData;
    fn data_mut(&mut self) -> &mut PrimitiveData;

    /// Control points as (bar_index, price) pairs
    fn points(&self) -> Vec<(f64, f64)>;
    fn set_points(&mut self, points: &[(f64, f64)]);

    /// Move primitive by delta
    fn translate(&mut self, bar_delta: f64, price_delta: f64);

    /// Render to context
    fn render(&self, ctx: &mut dyn RenderContext, is_selected: bool);

    /// Serialize to JSON
    fn to_json(&self) -> String;

    /// Clone as boxed trait object
    fn clone_box(&self) -> Box<dyn Primitive>;
}
```

### RenderContext Trait

Platform-agnostic rendering abstraction:

```rust
// crates/canvas/src/primitives/core/render.rs

pub trait RenderContext {
    // Chart dimensions
    fn chart_width(&self) -> f64;
    fn chart_height(&self) -> f64;

    // Coordinate conversion
    fn bar_to_x(&self, bar: f64) -> f64;
    fn price_to_y(&self, price: f64) -> f64;

    // Stroke style
    fn set_stroke_color(&mut self, color: &str);
    fn set_stroke_width(&mut self, width: f64);
    fn set_line_dash(&mut self, pattern: &[f64]);

    // Fill style
    fn set_fill_color(&mut self, color: &str);

    // Path operations
    fn begin_path(&mut self);
    fn move_to(&mut self, x: f64, y: f64);
    fn line_to(&mut self, x: f64, y: f64);
    fn close_path(&mut self);
    fn stroke(&mut self);
    fn fill(&mut self);

    // Shapes
    fn stroke_rect(&mut self, x: f64, y: f64, w: f64, h: f64);
    fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64);
    fn ellipse(&mut self, params: EllipseParams);
    fn arc(&mut self, cx: f64, cy: f64, radius: f64, start: f64, end: f64);

    // Curves
    fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64);
    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);

    // Text
    fn set_font(&mut self, font: &str);
    fn set_text_align(&mut self, align: TextAlign);
    fn fill_text(&mut self, text: &str, x: f64, y: f64);
    fn measure_text(&self, text: &str) -> f64;

    // State management
    fn save(&mut self);
    fn restore(&mut self);
    fn translate(&mut self, x: f64, y: f64);
    fn rotate(&mut self, angle: f64);

    // Device pixel ratio
    fn dpr(&self) -> f64;
}
```

---

## Indicator System

The indicator system renders pre-computed data with configurable visualizations.

```
┌─────────────────────────────────────────────────────────────┐
│                         Indicator                            │
├─────────────────────────────────────────────────────────────┤
│  id: "rsi_14"                                               │
│  name: "RSI (14)"                                           │
│  placement: SubPane { height_ratio: 0.15 }                  │
│  range: Fixed { min: 0.0, max: 100.0 }                      │
│  vectors: [                                                  │
│    IndicatorVector { values: [...], style: Line }           │
│  ]                                                           │
│  levels: [                                                   │
│    { value: 70.0, color: "#ef5350", style: "dashed" }       │
│    { value: 30.0, color: "#26a69a", style: "dashed" }       │
│  ]                                                           │
└─────────────────────────────────────────────────────────────┘
```

### Indicator Structure

```rust
// crates/canvas/src/model/indicators.rs

pub struct Indicator {
    pub id: String,
    pub name: String,
    pub placement: IndicatorPlacement,
    pub range: IndicatorRange,
    pub vectors: Vec<IndicatorVector>,
    pub levels: Vec<IndicatorLevel>,
}

pub enum IndicatorPlacement {
    Overlay,                              // On main chart
    OverlayBottom { height_ratio: f64 },  // Bottom of main chart
    SubPane { height_ratio: f64 },        // Separate pane below
}

pub enum IndicatorRange {
    Auto,                          // Auto-scale to data
    Fixed { min: f64, max: f64 },  // Fixed range (RSI: 0-100)
    Symmetric,                     // Symmetric around zero
    Price,                         // Same as price scale
}

pub struct IndicatorVector {
    pub values: Vec<f64>,
    pub style: VectorStyle,
}

pub enum VectorStyle {
    Line { color: String, width: f64 },
    Area { color: String, fill_alpha: f64 },
    Histogram { up_color: String, down_color: String },
    Markers { color: String, size: f64 },
    Band { color: String, fill_alpha: f64 },
}
```

### Preset Factory Pattern

```rust
impl Indicator {
    /// RSI preset - fixed 0-100 range, overbought/oversold levels
    pub fn rsi(id: &str, period: u32) -> Self {
        Self {
            id: id.to_string(),
            name: format!("RSI ({})", period),
            placement: IndicatorPlacement::subpane(0.15),
            range: IndicatorRange::fixed(0.0, 100.0),
            vectors: vec![IndicatorVector::line("#7E57C2")],
            levels: vec![
                IndicatorLevel::new(70.0, "#ef5350").dashed(),
                IndicatorLevel::new(30.0, "#26a69a").dashed(),
            ],
            ..Default::default()
        }
    }

    /// Bollinger Bands preset - 3 vectors (middle, upper, lower)
    pub fn bollinger(id: &str, period: u32) -> Self {
        Self {
            id: id.to_string(),
            name: format!("BB ({})", period),
            placement: IndicatorPlacement::overlay(),
            range: IndicatorRange::price(),
            vectors: vec![
                IndicatorVector::line("#2196F3"),      // Middle
                IndicatorVector::line("#90CAF9"),      // Upper
                IndicatorVector::line("#90CAF9"),      // Lower
            ],
            ..Default::default()
        }
    }
}
```

---

## Theme System

The theme system provides consistent styling across the chart.

```
┌─────────────────────────────────────────────────────────────┐
│                         UITheme                              │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   colors    │  │    chart    │  │   series    │         │
│  │ (UI colors) │  │  (bg,grid)  │  │  (candles)  │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└─────────────────────────────────────────────────────────────┘
```

### Theme Types

```rust
// crates/canvas/src/theme/ui_theme.rs

/// Static theme with compile-time strings (zero-copy)
pub struct UITheme {
    pub name: &'static str,
    pub colors: UIColors,     // Toolbar, buttons
    pub chart: ChartColors,   // Background, grid, scales
    pub series: SeriesColors, // Candles, lines
    pub font: FontTheme,      // Font family, sizes
}

/// Runtime-modifiable theme with JSON support
pub struct RuntimeTheme {
    pub name: String,
    pub colors: RuntimeUIColors,
    pub chart: RuntimeChartColors,
    pub series: RuntimeSeriesColors,
}

impl RuntimeTheme {
    pub fn from_json(json: &str) -> Option<Self>;
    pub fn to_json(&self) -> String;
}
```

### Theme Presets

```rust
impl UITheme {
    pub fn dark() -> Self {
        Self {
            name: "Dark",
            chart: ChartColors {
                background: "#131722",
                grid_line: "#2a2e3999",
                scale_bg: "#1e222d",
                scale_text: "#b2b5be",
                crosshair_line: "#758696",
            },
            series: SeriesColors {
                candle_up_body: "#26a69a",
                candle_down_body: "#ef5350",
                candle_up_wick: "#26a69a",
                candle_down_wick: "#ef5350",
                // ...
            },
            // ...
        }
    }

    pub fn light() -> Self { ... }
    pub fn high_contrast() -> Self { ... }
    pub fn cyberpunk() -> Self { ... }
}
```

---

## Layout System

The layout system handles multi-chart and subpane arrangements.

### Multichart Layouts

```
Single (1x1)           Horizontal Split (1x2)      Quad (2x2)
┌─────────────┐        ┌──────┬──────┐            ┌──────┬──────┐
│             │        │      │      │            │      │      │
│      0      │        │  0   │  1   │            │  0   │  1   │
│             │        │      │      │            │      │      │
└─────────────┘        └──────┴──────┘            ├──────┼──────┤
                                                  │      │      │
                                                  │  2   │  3   │
                                                  │      │      │
                                                  └──────┴──────┘

One Plus Three (1+3)   Triple Vertical (3x1)
┌────────┬────┐        ┌─────────────┐
│        │ 1  │        │      0      │
│   0    ├────┤        ├─────────────┤
│        │ 2  │        │      1      │
│        ├────┤        ├─────────────┤
│        │ 3  │        │      2      │
└────────┴────┘        └─────────────┘
```

### Layout Structure

```rust
// crates/canvas/src/layout/multichart.rs

pub struct MultichartLayout {
    pub name: String,
    pub rows: usize,
    pub cols: usize,
    pub gap: f64,
    pub cells: Vec<LayoutCell>,
    pub shared_time_scale: bool,
    pub sync_crosshair: bool,
}

pub struct LayoutCell {
    pub id: CellId,
    pub row: usize,
    pub col: usize,
    pub row_span: usize,
    pub col_span: usize,
    pub show_price_scale: bool,
    pub show_time_scale: bool,
}

impl LayoutCell {
    pub fn bounds(&self, total_width: f64, total_height: f64, rows: usize, cols: usize, gap: f64)
        -> CellBounds
    {
        // Calculate pixel bounds for this cell
    }
}
```

### Subpane Layout

```
Main Chart (main_ratio = 0.65)
┌─────────────────────────────────────────────────────┬────────┐
│                                                     │ Price  │
│                    Candlesticks                     │ Scale  │
│                                                     │        │
├─────────────────────────────────────────────────────┼────────┤
│  Volume (height_ratio = 0.15)                       │        │
├─────────────────────────────────────────────────────┼────────┤
│  RSI (height_ratio = 0.10)                          │ 0-100  │
├─────────────────────────────────────────────────────┼────────┤
│  MACD (height_ratio = 0.10)                         │ Auto   │
├─────────────────────────────────────────────────────┴────────┤
│                       Time Scale                             │
└──────────────────────────────────────────────────────────────┘
```

---

## Bindings Architecture

The bindings wrap the core Rust library for Python and JavaScript.

```
                    ┌─────────────────────────────┐
                    │     Core Rust Library       │
                    │     (zengeld-canvas)        │
                    └──────────────┬──────────────┘
                                   │
          ┌────────────────────────┼────────────────────────┐
          │                        │                        │
          ▼                        ▼                        ▼
┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────┐
│    canvas-wasm      │  │     canvas-py       │  │    Direct Rust      │
│   (wasm-bindgen)    │  │      (PyO3)         │  │                     │
├─────────────────────┤  ├─────────────────────┤  ├─────────────────────┤
│ Naming: camelCase   │  │ Naming: snake_case  │  │ Naming: snake_case  │
│ Types: JsValue      │  │ Types: PyObject     │  │ Types: native       │
│ Errors: JsError     │  │ Errors: PyErr       │  │ Errors: Result      │
└─────────────────────┘  └─────────────────────┘  └─────────────────────┘
          │                        │                        │
          ▼                        ▼                        ▼
┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────┐
│   JavaScript/TS     │  │      Python         │  │   Rust Application  │
│   import { Chart }  │  │   from zengeld_     │  │   use zengeld_      │
│   from 'zengeld-    │  │   canvas import     │  │   canvas::Chart;    │
│   canvas';          │  │   Chart             │  │                     │
└─────────────────────┘  └─────────────────────┘  └─────────────────────┘
```

### Wrapper Pattern

Both bindings follow the same pattern:

```rust
// Wrapper struct holds the Rust type
pub struct PyChart {
    inner: Option<RustChart>,
}

// Take/put pattern for builder methods
impl PyChart {
    fn take_inner(&mut self) -> RustChart {
        self.inner.take().expect("Chart already consumed")
    }

    fn put_inner(&mut self, chart: RustChart) {
        self.inner = Some(chart);
    }
}

// Builder method pattern
fn sma(&mut self, period: usize, color: &str) {
    let chart = self.take_inner().sma(period, color);
    self.put_inner(chart);
}
```

### Naming Convention Mapping

| Rust | Python | JavaScript |
|------|--------|------------|
| `bar_to_x` | `bar_to_x` | `barToX` |
| `set_bar_count` | `set_bar_count` | `setBarCount` |
| `scroll_to_end` | `scroll_to_end` | `scrollToEnd` |
| `is_bullish` | `is_bullish` | `isBullish` |

---

## Key Files Reference

| File | Lines | Description |
|------|-------|-------------|
| `api/chart.rs` | ~700 | Chart builder and renderer |
| `coords/viewport.rs` | ~400 | Main coordinate system |
| `coords/price_scale.rs` | ~550 | Price axis with nice numbers |
| `primitives/registry.rs` | ~350 | Primitive factory pattern |
| `primitives/core/render.rs` | ~480 | RenderContext trait |
| `model/indicators.rs` | ~1800 | Indicator presets (60+) |
| `render/engine/svg_backend.rs` | ~500 | SVG output generation |
| `canvas-wasm/src/lib.rs` | ~2250 | Complete WASM bindings |
| `canvas-py/src/lib.rs` | ~2050 | Complete Python bindings |

---

## Further Reading

- [CONTRIBUTING.md](./CONTRIBUTING.md) - How to contribute
- [crates/canvas-wasm/CONTRIBUTING.md](./crates/canvas-wasm/CONTRIBUTING.md) - WASM specifics
- [crates/canvas-py/CONTRIBUTING.md](./crates/canvas-py/CONTRIBUTING.md) - Python specifics
