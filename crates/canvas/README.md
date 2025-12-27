# zengeld-canvas

**High-performance SVG chart rendering engine for financial data visualization**

[![Crates.io](https://img.shields.io/crates/v/zengeld-canvas.svg)](https://crates.io/crates/zengeld-canvas)
[![Documentation](https://docs.rs/zengeld-canvas/badge.svg)](https://docs.rs/zengeld-canvas)
[![License](https://img.shields.io/crates/l/zengeld-canvas.svg)](LICENSE)

A platform-agnostic rendering library for financial charts. Built in Rust with zero runtime dependencies, designed to work seamlessly across native platforms, WebAssembly, and Python bindings.

## Features

- **80+ Drawing Primitives** - Comprehensive set of technical analysis tools:
  - Trend lines, rays, extended lines, arrows
  - Fibonacci retracements, extensions, fans, arcs, spirals, time zones
  - Gann tools (fan, box, square, fixed square)
  - Pitchforks (standard, Schiff, modified Schiff, inside)
  - Patterns (XABCD, Cypher, Head & Shoulders, ABCD, Three Drives, Elliott waves)
  - Channels (parallel, regression, disjoint angle)
  - Shapes (rectangles, ellipses, triangles, arcs, polylines, paths)
  - Annotations (text, notes, callouts, price labels, flags, tables)
  - Trading tools (long/short positions, price ranges, date ranges)

- **12 Series Types** - Complete data visualization:
  - Candlestick, OHLC bars
  - Line, Step line, Area, Baseline
  - Histogram, Columns
  - High-Low, Range area
  - Markers, Circles

- **Platform Agnostic** - `RenderContext` trait abstraction for any rendering backend
- **Coordinate Systems** - Built-in viewport, price scale, and time scale management
- **Layout System** - Multi-pane charts, sub-panes, multichart grids (10+ presets)
- **Zero Dependencies** - Only serde/serde_json for serialization
- **High Performance** - Optimized for real-time chart rendering

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
zengeld-canvas = "0.1"
```

## Quick Start

```rust
use zengeld_canvas::{
    RenderContext, Viewport, PriceScale, TimeScale,
    CandlestickData, Theme,
};

// Implement RenderContext for your platform (Canvas2D, SVG, WebGL, etc.)
struct MyRenderer { /* ... */ }

impl RenderContext for MyRenderer {
    fn chart_width(&self) -> f64 { 800.0 }
    fn chart_height(&self) -> f64 { 600.0 }
    fn dpr(&self) -> f64 { 1.0 }

    fn bar_to_x(&self, bar: f64) -> f64 { /* coordinate transform */ }
    fn price_to_y(&self, price: f64) -> f64 { /* coordinate transform */ }

    // Drawing operations
    fn begin_path(&mut self) { }
    fn move_to(&mut self, x: f64, y: f64) { }
    fn line_to(&mut self, x: f64, y: f64) { }
    fn stroke(&mut self) { }
    // ... see docs for full trait
}

// Create chart components
let viewport = Viewport::new(800.0, 600.0);
let theme = Theme::dark();
```

## Architecture

```
zengeld-canvas/
├── core/           # Foundational types (Bar, Theme, ChartConfig)
├── coords/         # Coordinate systems (Viewport, PriceScale, TimeScale)
├── model/          # Data models (series, overlays, annotations)
├── primitives/     # 80+ interactive drawing tools
├── layout/         # Pane system, multichart grids
├── render/         # Rendering engine, batch operations
└── api/            # High-level chart API
```

### Module Overview

| Module | Description |
|--------|-------------|
| `core` | Bar data, Theme, configuration types, color parsing, math utilities |
| `coords` | Viewport management, price/time scale calculations, tick formatting |
| `model` | Series options, overlay configs, marker/annotation data |
| `primitives` | Drawing tools with `PrimitiveTrait`, registry pattern, control points |
| `layout` | `PaneManager` for multi-pane charts, `MultichartLayout` for grids |
| `render` | `RenderContext` trait, render operations, text rendering |

## Drawing Primitives

All primitives implement the `PrimitiveTrait`:

```rust
use zengeld_canvas::primitives::{PrimitiveTrait, PrimitiveRegistry};

// Get all available primitives
let registry = PrimitiveRegistry::new();
for metadata in registry.all_metadata() {
    println!("{}: {}", metadata.type_id, metadata.display_name);
}

// Create a primitive via factory
let trend_line = registry.create("trend_line", &[(0.0, 100.0), (50.0, 150.0)], "#2962ff");
```

### Primitive Categories

| Category | Examples |
|----------|----------|
| Lines | Trend Line, Ray, Extended Line, Horizontal/Vertical Lines, Cross Line |
| Fibonacci | Retracement, Extension, Fan, Arcs, Spiral, Time Zones, Wedge, Channel |
| Gann | Fan, Box, Square, Fixed Square |
| Pitchforks | Standard, Schiff, Modified Schiff, Inside Pitchfork |
| Patterns | XABCD, Cypher, Head & Shoulders, ABCD, Triangle, Three Drives |
| Elliott | Impulse Wave, Corrective Wave, Triangle Wave, Combination, Degree |
| Channels | Parallel, Flat Top/Bottom, Disjoint Angle, Regression |
| Shapes | Rectangle, Ellipse, Triangle, Arc, Polyline, Path, Curve, Brush |
| Ranges | Price Range, Date Range, Date/Price Range, Bars Pattern |
| Positions | Long Position, Short Position, Forecast, Projection |
| Annotations | Text, Note, Callout, Price Label, Signpost, Flag, Comment, Table |
| Arrows | Arrow Marker, Arrow Line, Arrow Up/Down |
| Icons | Emoji, Image |
| Signals | Entry/Exit signals, Crossover, Breakdown, Divergence, Pattern Match |

## Series Types

```rust
use zengeld_canvas::{CandlestickData, LineData, AreaData};

// Candlestick data
let candle = CandlestickData {
    time: 1703721600,
    open: 100.0,
    high: 105.0,
    low: 98.0,
    close: 103.0,
};

// Line data
let point = LineData {
    time: 1703721600,
    value: 103.0,
};
```

## Layouts

### Multi-Pane Charts

```rust
use zengeld_canvas::{PaneManager, MAIN_PANE};

let mut manager = PaneManager::new();
manager.add_pane("volume", 0.2); // Add volume pane at 20% height
manager.add_pane("rsi", 0.15);   // Add RSI pane at 15% height
```

### Multichart Grids

```rust
use zengeld_canvas::MultichartLayout;

// Built-in presets
let quad = MultichartLayout::quad();           // 2x2 grid
let six = MultichartLayout::six_pack();        // 2x3 grid
let custom = MultichartLayout::grid(3, 4);     // 3x4 grid
```

## Platform Targets

| Platform | Method | Status |
|----------|--------|--------|
| Rust (native) | Direct dependency | Available |
| WebAssembly | wasm-bindgen | Planned |
| Python | PyO3 bindings | Planned |
| Node.js | WASM + npm | Planned |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Author

**zengeld**

---

*Built with Rust for speed and reliability.*
