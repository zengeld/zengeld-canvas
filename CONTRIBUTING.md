# Contributing to zengeld-canvas

Thank you for your interest in contributing to zengeld-canvas! This guide will help you get started.

## Table of Contents

- [Project Structure](#project-structure)
- [Development Setup](#development-setup)
- [Adding a New Primitive](#adding-a-new-primitive)
- [Adding a New Indicator](#adding-a-new-indicator)
- [Adding a New Series Type](#adding-a-new-series-type)
- [Code Style Guidelines](#code-style-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing Requirements](#testing-requirements)

---

## Project Structure

```
zengeld-canvas/
├── Cargo.toml                    # Workspace manifest
├── crates/
│   ├── canvas/                   # Core Rust library
│   │   ├── src/
│   │   │   ├── lib.rs            # Public API exports
│   │   │   ├── api/              # High-level Chart builder API
│   │   │   │   ├── chart.rs      # ChartRenderer, Chart builder
│   │   │   │   └── config.rs     # ChartConfig, SeriesConfig, etc.
│   │   │   ├── coords/           # Coordinate systems
│   │   │   │   ├── viewport.rs   # Main Viewport (combines TimeScale + PriceScale)
│   │   │   │   ├── time_scale.rs # X-axis: bar index <-> pixel
│   │   │   │   └── price_scale.rs# Y-axis: price <-> pixel (with nice numbers)
│   │   │   ├── core/             # Core types and constants
│   │   │   │   ├── types.rs      # Bar, Theme, layout constants
│   │   │   │   └── config.rs     # Color, Font, Series configs
│   │   │   ├── model/            # Data models
│   │   │   │   ├── series/       # Series types (Candlestick, Line, Area...)
│   │   │   │   ├── indicators.rs # Indicator system (60+ presets)
│   │   │   │   ├── overlays/     # Overlay annotations
│   │   │   │   └── annotations/  # Text, markers, price lines
│   │   │   ├── primitives/       # Drawing primitives (90+ types)
│   │   │   │   ├── mod.rs        # Primitive trait, types
│   │   │   │   ├── registry.rs   # PrimitiveRegistry (factory pattern)
│   │   │   │   ├── core/         # Core abstractions (render.rs, types.rs)
│   │   │   │   └── catalog/      # All primitive implementations
│   │   │   │       ├── lines/    # TrendLine, HorizontalLine, Ray...
│   │   │   │       ├── channels/ # ParallelChannel, RegressionTrend...
│   │   │   │       ├── shapes/   # Rectangle, Circle, Triangle...
│   │   │   │       ├── fibonacci/# FibRetracement, FibChannel...
│   │   │   │       ├── gann/     # GannBox, GannFan, GannSquare...
│   │   │   │       ├── pitchforks/# Pitchfork, Schiff, ModifiedSchiff...
│   │   │   │       ├── patterns/ # XABCD, HeadShoulders, Elliott...
│   │   │   │       └── ...       # 19 categories total
│   │   │   ├── render/           # Rendering engine
│   │   │   │   ├── engine/       # RenderBackend trait, SvgBackend
│   │   │   │   └── chart/        # Chart rendering (candlesticks, lines)
│   │   │   ├── layout/           # Layout system
│   │   │   │   ├── multichart.rs # Grid layouts (1x1, 2x2, 1+3...)
│   │   │   │   └── panes.rs      # Subpane layout for indicators
│   │   │   └── theme/            # Theme system
│   │   │       └── ui_theme.rs   # UITheme, RuntimeTheme
│   │   └── Cargo.toml
│   │
│   ├── canvas-wasm/              # WebAssembly bindings
│   │   ├── src/lib.rs            # wasm-bindgen exports
│   │   └── Cargo.toml
│   │
│   └── canvas-py/                # Python bindings
│       ├── src/lib.rs            # PyO3 exports
│       └── Cargo.toml
└── README.md
```

### Key Modules

| Module | Purpose |
|--------|---------|
| `api/chart.rs` | High-level `Chart` builder and `ChartRenderer` |
| `coords/viewport.rs` | Main coordinate system combining TimeScale + PriceScale |
| `primitives/registry.rs` | Factory pattern for creating primitives by type_id |
| `primitives/core/render.rs` | `RenderContext` trait - platform-agnostic rendering |
| `model/indicators.rs` | `Indicator` struct with 60+ presets |
| `render/engine/svg_backend.rs` | SVG output generation |

---

## Development Setup

### Prerequisites

- **Rust** 1.70+ (install via [rustup](https://rustup.rs/))
- **wasm-pack** (for WASM bindings): `cargo install wasm-pack`
- **maturin** (for Python bindings): `pip install maturin`
- **Python** 3.8+ with venv

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/your-org/zengeld-canvas.git
cd zengeld-canvas

# Build the core library
cargo build

# Run tests
cargo test

# Check for warnings
cargo clippy

# Format code
cargo fmt
```

### Building WASM

```bash
cd crates/canvas-wasm

# Build for npm
wasm-pack build --target web

# Or for Node.js
wasm-pack build --target nodejs

# Output will be in pkg/
```

### Building Python

```bash
cd crates/canvas-py

# Create virtual environment
python -m venv .venv
source .venv/bin/activate  # or .venv\Scripts\activate on Windows

# Build and install in development mode
maturin develop

# Test the installation
python -c "import zengeld_canvas; print(zengeld_canvas.version())"
```

---

## Adding a New Primitive

Primitives are drawing objects like lines, shapes, and patterns. Follow these steps:

### Step 1: Create the Primitive File

Create a new file in the appropriate category under `crates/canvas/src/primitives/catalog/`.

For example, to add a new line type `double_line.rs`:

```
crates/canvas/src/primitives/catalog/lines/double_line.rs
```

### Step 2: Implement the Primitive Struct

Use `TrendLine` as a reference (see `crates/canvas/src/primitives/catalog/lines/trend_line.rs`):

```rust
//! Double Line primitive - two parallel lines

use super::super::{
    LineStyle, Primitive, PrimitiveColor, PrimitiveData, PrimitiveKind,
    PrimitiveMetadata, RenderContext, crisp,
};
use serde::{Deserialize, Serialize};

/// Double Line - two parallel lines with configurable spacing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoubleLine {
    /// Common primitive data (color, width, style, text, etc.)
    pub data: PrimitiveData,
    /// First point bar index (f64 for sub-bar precision)
    pub bar1: f64,
    /// First point price
    pub price1: f64,
    /// Second point bar index
    pub bar2: f64,
    /// Second point price
    pub price2: f64,
    /// Spacing between the two lines in pixels
    pub spacing: f64,
}

impl DoubleLine {
    /// Create a new double line
    pub fn new(bar1: f64, price1: f64, bar2: f64, price2: f64, color: &str) -> Self {
        Self {
            data: PrimitiveData {
                type_id: "double_line".to_string(),
                display_name: "Double Line".to_string(),
                color: PrimitiveColor::new(color),
                width: 2.0,
                ..Default::default()
            },
            bar1,
            price1,
            bar2,
            price2,
            spacing: 10.0,
        }
    }
}
```

### Step 3: Implement the Primitive Trait

The `Primitive` trait defines the interface for all primitives:

```rust
impl Primitive for DoubleLine {
    fn type_id(&self) -> &'static str {
        "double_line"
    }

    fn display_name(&self) -> &str {
        &self.data.display_name
    }

    fn kind(&self) -> PrimitiveKind {
        PrimitiveKind::Line
    }

    fn data(&self) -> &PrimitiveData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut PrimitiveData {
        &mut self.data
    }

    fn points(&self) -> Vec<(f64, f64)> {
        vec![(self.bar1, self.price1), (self.bar2, self.price2)]
    }

    fn set_points(&mut self, points: &[(f64, f64)]) {
        if points.len() >= 2 {
            self.bar1 = points[0].0;
            self.price1 = points[0].1;
            self.bar2 = points[1].0;
            self.price2 = points[1].1;
        }
    }

    fn translate(&mut self, bar_delta: f64, price_delta: f64) {
        self.bar1 += bar_delta;
        self.bar2 += bar_delta;
        self.price1 += price_delta;
        self.price2 += price_delta;
    }

    fn render(&self, ctx: &mut dyn RenderContext, _is_selected: bool) {
        let dpr = ctx.dpr();

        // Convert to screen coordinates
        let x1 = ctx.bar_to_x(self.bar1);
        let y1 = ctx.price_to_y(self.price1);
        let x2 = ctx.bar_to_x(self.bar2);
        let y2 = ctx.price_to_y(self.price2);

        // Calculate perpendicular offset for second line
        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();
        let (nx, ny) = if len > 0.001 {
            (-dy / len * self.spacing, dx / len * self.spacing)
        } else {
            (0.0, self.spacing)
        };

        // Set stroke style
        ctx.set_stroke_color(&self.data.color.stroke);
        ctx.set_stroke_width(self.data.width);
        ctx.set_line_style(self.data.style);

        // Draw first line
        ctx.begin_path();
        ctx.move_to(crisp(x1, dpr), crisp(y1, dpr));
        ctx.line_to(crisp(x2, dpr), crisp(y2, dpr));
        ctx.stroke();

        // Draw second line (offset)
        ctx.begin_path();
        ctx.move_to(crisp(x1 + nx, dpr), crisp(y1 + ny, dpr));
        ctx.line_to(crisp(x2 + nx, dpr), crisp(y2 + ny, dpr));
        ctx.stroke();

        // Reset line dash
        ctx.set_line_dash(&[]);
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    fn clone_box(&self) -> Box<dyn Primitive> {
        Box::new(self.clone())
    }
}
```

### Step 4: Create Factory and Metadata

Add the factory function and metadata at the bottom of your file:

```rust
// =============================================================================
// Factory Registration
// =============================================================================

/// Create double line from points
fn create_double_line(points: &[(f64, f64)], color: &str) -> Box<dyn Primitive> {
    let (bar1, price1) = points.first().copied().unwrap_or((0.0, 0.0));
    let (bar2, price2) = points.get(1).copied().unwrap_or((bar1, price1));
    Box::new(DoubleLine::new(bar1, price1, bar2, price2, color))
}

/// Get metadata for registry
pub fn metadata() -> PrimitiveMetadata {
    PrimitiveMetadata {
        type_id: "double_line",
        display_name: "Double Line",
        kind: PrimitiveKind::Line,
        factory: create_double_line,
        supports_text: true,
        has_levels: false,
        has_points_config: false,
    }
}
```

### Step 5: Register in the Module

1. Add to the category's `mod.rs` (e.g., `primitives/catalog/lines/mod.rs`):

```rust
pub mod double_line;
```

2. Register in `primitives/registry.rs` in the `register_builtins()` method:

```rust
fn register_builtins(&mut self) {
    // Lines
    self.register(super::catalog::lines::trend_line::metadata());
    self.register(super::catalog::lines::double_line::metadata()); // <-- Add here
    // ...
}
```

### Step 6: Add Bindings (Optional)

To expose the primitive in Python/JS, add methods to the Chart class in the bindings:

**WASM** (`crates/canvas-wasm/src/lib.rs`):
```rust
#[wasm_bindgen(js_name = doubleLine)]
pub fn double_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, spacing: f64) {
    // Use PrimitiveConfig or create directly
    // ...
}
```

**Python** (`crates/canvas-py/src/lib.rs`):
```rust
fn double_line(&mut self, p1: (f64, f64), p2: (f64, f64), spacing: f64) {
    // ...
}
```

---

## Adding a New Indicator

Indicators are visualization configurations for computed data. The library does NOT compute indicators - it only renders pre-computed values.

### Step 1: Add a Preset Method

Add a new preset method to `crates/canvas/src/model/indicators.rs`:

```rust
impl Indicator {
    // ... existing methods ...

    /// Custom RSI with smoothing (example)
    pub fn smooth_rsi(id: &str, period: u32, smoothing: u32) -> Self {
        Self {
            id: id.to_string(),
            name: format!("Smooth RSI({}, {})", period, smoothing),
            placement: IndicatorPlacement::subpane(0.15),
            range: IndicatorRange::fixed(0.0, 100.0),
            vectors: vec![
                IndicatorVector::line("#7E57C2"), // Main line
                IndicatorVector::line("#B39DDB"), // Signal line
            ],
            levels: vec![
                IndicatorLevel::new(70.0, "#ef5350").dashed(), // Overbought
                IndicatorLevel::new(30.0, "#26a69a").dashed(), // Oversold
            ],
            ..Default::default()
        }
    }
}
```

### Step 2: Understand VectorStyle Options

Indicators support multiple visualization styles:

```rust
pub enum VectorStyle {
    /// Line chart
    Line { color: String, width: f64 },
    /// Filled area below line
    Area { color: String, fill_alpha: f64 },
    /// Histogram bars
    Histogram { up_color: String, down_color: String },
    /// Dots/markers
    Markers { color: String, size: f64 },
    /// Band (upper/lower)
    Band { color: String, fill_alpha: f64 },
}
```

### Step 3: Add Bindings

**WASM** (`crates/canvas-wasm/src/lib.rs`):
```rust
#[wasm_bindgen(js_name = smoothRsi)]
pub fn smooth_rsi(&mut self, period: usize, smoothing: usize) {
    let id = format!("smooth_rsi_{}_{}", period, smoothing);
    let indicator = Indicator::smooth_rsi(&id, period as u32, smoothing as u32);
    let chart = self.take_inner().indicator(indicator);
    self.put_inner(chart);
}
```

**Python** (`crates/canvas-py/src/lib.rs`):
```rust
fn smooth_rsi(&mut self, period: usize, smoothing: usize) {
    let id = format!("smooth_rsi_{}_{}", period, smoothing);
    let indicator = Indicator::smooth_rsi(&id, period as u32, smoothing as u32);
    let chart = self.take_inner().indicator(indicator);
    self.put_inner(chart);
}
```

---

## Adding a New Series Type

Series types determine how price data is visualized.

### Step 1: Add the Enum Variant

In `crates/canvas/src/model/series/types.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SeriesType {
    #[default]
    Candlestick,
    Line,
    Area,
    HeikinAshi,      // <-- Add new type
    // ...
}
```

### Step 2: Implement Rendering

Add rendering logic in `crates/canvas/src/render/chart/`:

1. Create a new render function or extend existing ones
2. Handle the new series type in `ChartRenderer::render_main_series()`

### Step 3: Add to Chart Builder

In `crates/canvas/src/api/chart.rs`, add a builder method:

```rust
impl Chart {
    pub fn heikin_ashi(mut self) -> Self {
        self.series_type = SeriesType::HeikinAshi;
        self
    }
}
```

---

## Code Style Guidelines

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Use descriptive names: `bar_to_x` not `b2x`
- Document public APIs with `///` doc comments
- Add examples in doc comments where helpful

### Naming Conventions

| Context | Style | Example |
|---------|-------|---------|
| Rust functions | snake_case | `bar_to_x()` |
| Rust structs | PascalCase | `TrendLine` |
| JS methods | camelCase | `setBarCount()` |
| Python methods | snake_case | `set_bar_count()` |
| Type IDs | snake_case | `"trend_line"` |

### File Organization

- One primitive per file
- Group related primitives in subdirectories
- Keep files under 500 lines when possible
- Use `mod.rs` to organize exports

---

## Pull Request Process

1. **Fork** the repository
2. **Create a branch** for your feature: `git checkout -b feature/my-new-primitive`
3. **Make changes** following the guidelines above
4. **Add tests** for new functionality
5. **Run checks**:
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```
6. **Commit** with a descriptive message
7. **Push** to your fork
8. **Open a PR** with:
   - Clear description of changes
   - Screenshots for visual features
   - Reference to related issues

### PR Checklist

- [ ] Code follows style guidelines
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] `cargo fmt` run
- [ ] `cargo clippy` passes
- [ ] All tests pass

---

## Testing Requirements

### Unit Tests

Add tests in the same file using `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_line_creation() {
        let line = DoubleLine::new(0.0, 100.0, 10.0, 110.0, "#2196F3");
        assert_eq!(line.type_id(), "double_line");
        assert_eq!(line.points().len(), 2);
    }

    #[test]
    fn test_translate() {
        let mut line = DoubleLine::new(0.0, 100.0, 10.0, 110.0, "#2196F3");
        line.translate(5.0, 10.0);
        assert_eq!(line.bar1, 5.0);
        assert_eq!(line.price1, 110.0);
    }
}
```

### Integration Tests

For larger features, add integration tests in `tests/`:

```rust
// tests/primitives_test.rs
use zengeld_canvas::primitives::PrimitiveRegistry;

#[test]
fn test_registry_has_double_line() {
    let registry = PrimitiveRegistry::global().read().unwrap();
    assert!(registry.get("double_line").is_some());
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_double_line

# Run tests with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

---

## Questions?

- Open an issue for bugs or feature requests
- Check existing issues before creating new ones
- Join discussions in pull requests

Thank you for contributing!
