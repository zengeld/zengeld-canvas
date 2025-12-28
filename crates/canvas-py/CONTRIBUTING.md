# Contributing to canvas-py

This guide covers how to add and modify Python bindings using PyO3.

## Table of Contents

- [Adding a New Python Class](#adding-a-new-python-class)
- [PyO3 Patterns](#pyo3-patterns)
- [Naming Conventions](#naming-conventions)
- [Common Patterns](#common-patterns)

---

## Adding a New Python Class

### Step 1: Define the Wrapper Struct

```rust
use pyo3::prelude::*;
use ::zengeld_canvas::SomeRustType;

/// Docstring becomes Python class docstring
#[pyclass(name = "MyClass")]
pub struct PyMyClass {
    inner: SomeRustType,
}
```

### Step 2: Implement Methods

```rust
#[pymethods]
impl PyMyClass {
    /// Constructor - becomes __init__
    #[new]
    #[pyo3(signature = (value, name=None))]
    fn new(value: f64, name: Option<String>) -> Self {
        Self {
            inner: SomeRustType::new(value, name.as_deref()),
        }
    }

    /// Getter property
    #[getter]
    fn value(&self) -> f64 {
        self.inner.value()
    }

    /// Setter property
    #[setter]
    fn set_value(&mut self, value: f64) {
        self.inner.set_value(value);
    }

    /// Regular method
    fn calculate(&self, factor: f64) -> f64 {
        self.inner.calculate(factor)
    }

    /// __repr__ for printing
    fn __repr__(&self) -> String {
        format!("MyClass(value={})", self.inner.value())
    }
}
```

### Step 3: Static Methods and Class Methods

```rust
#[pymethods]
impl PyMyClass {
    /// Static method (no self)
    #[staticmethod]
    fn from_json(json: &str) -> Option<Self> {
        SomeRustType::from_json(json).map(|inner| Self { inner })
    }

    /// Class method (receives cls)
    #[classmethod]
    fn create_default(_cls: &Bound<'_, PyType>) -> Self {
        Self {
            inner: SomeRustType::default(),
        }
    }
}
```

### Step 4: Register in Module

At the bottom of `lib.rs`, add to the module:

```rust
#[pymodule]
fn zengeld_canvas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<PyBar>()?;
    m.add_class::<PyChart>()?;
    m.add_class::<PyMyClass>()?;  // <-- Add here
    // ...
    Ok(())
}
```

### Full Example

See how `PyChart` is implemented in `src/lib.rs`:

```rust
// src/lib.rs (lines ~96-200)

#[pyclass(name = "Chart")]
pub struct PyChart {
    inner: Option<RustChart>,
}

impl PyChart {
    fn take_inner(&mut self) -> RustChart {
        self.inner.take().expect("Chart already consumed")
    }
    fn put_inner(&mut self, chart: RustChart) {
        self.inner = Some(chart);
    }
}

#[pymethods]
impl PyChart {
    #[new]
    #[pyo3(signature = (width, height))]
    fn new(width: u32, height: u32) -> Self {
        Self {
            inner: Some(RustChart::new(width, height)),
        }
    }

    /// Set device pixel ratio for high-DPI displays
    fn dpr(&mut self, dpr: f64) {
        let chart = self.take_inner().dpr(dpr);
        self.put_inner(chart);
    }

    /// Set OHLCV bar data
    fn bars(&mut self, bars: Vec<PyBar>) {
        let rust_bars: Vec<Bar> = bars.iter().map(|b| b.to_rust()).collect();
        let chart = self.take_inner().bars(&rust_bars);
        self.put_inner(chart);
    }

    /// Render chart to SVG string
    fn render_svg(&self) -> String {
        self.inner
            .as_ref()
            .map(|c| c.render_svg())
            .unwrap_or_default()
    }
}
```

---

## PyO3 Patterns

### Signature Specification

Use `#[pyo3(signature = ...)]` for optional/default parameters:

```rust
#[pymethods]
impl PyChart {
    #[new]
    #[pyo3(signature = (width, height))]
    fn new(width: u32, height: u32) -> Self { ... }

    #[pyo3(signature = (bar_index, price, label=None))]
    fn buy_signal(&mut self, bar_index: usize, price: f64, label: Option<String>) {
        // label is optional in Python
    }

    #[pyo3(signature = (period=14))]
    fn rsi(&mut self, period: usize) {
        // period defaults to 14
    }
}
```

### Error Handling

```rust
use pyo3::exceptions::PyValueError;

#[pymethods]
impl PyMyClass {
    fn parse(&mut self, json: &str) -> PyResult<()> {
        self.inner.parse(json)
            .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))
    }
}
```

### Converting Types

```rust
impl PyBar {
    /// Internal conversion to Rust type
    fn to_rust(&self) -> Bar {
        self.inner  // If Copy
    }
}

impl From<Bar> for PyBar {
    fn from(bar: Bar) -> Self {
        Self { inner: bar }
    }
}
```

### Properties with Getters/Setters

```rust
#[pymethods]
impl PyBar {
    #[getter]
    fn timestamp(&self) -> i64 {
        self.inner.timestamp
    }

    #[getter]
    fn open(&self) -> f64 {
        self.inner.open
    }

    // Read-only: no setter

    // Or with setter:
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    #[setter]
    fn set_name(&mut self, name: String) {
        self.inner.name = name;
    }
}
```

### Magic Methods

```rust
#[pymethods]
impl PyBar {
    /// String representation: repr(obj)
    fn __repr__(&self) -> String {
        format!(
            "Bar(ts={}, o={}, h={}, l={}, c={}, v={})",
            self.inner.timestamp,
            self.inner.open,
            self.inner.high,
            self.inner.low,
            self.inner.close,
            self.inner.volume
        )
    }

    /// String conversion: str(obj)
    fn __str__(&self) -> String {
        format!("Bar({}, {})", self.inner.timestamp, self.inner.close)
    }

    /// Boolean conversion: bool(obj)
    fn __bool__(&self) -> bool {
        self.inner.volume > 0.0
    }

    /// Length: len(obj)
    fn __len__(&self) -> usize {
        5  // OHLCV fields
    }
}
```

---

## Naming Conventions

All public APIs use **snake_case** for Python:

| Rust Method | Python Method |
|-------------|---------------|
| `bar_to_x()` | `bar_to_x()` |
| `render_svg()` | `render_svg()` |
| `is_bullish()` | `is_bullish()` |

**Class Naming:**

```rust
// Rust wrapper uses Py prefix internally
pub struct PyChart { ... }

// Python sees just "Chart"
#[pyclass(name = "Chart")]
```

---

## Common Patterns

### Working with Lists

```rust
#[pymethods]
impl PyChart {
    /// Accept Python list of Bars
    fn bars(&mut self, bars: Vec<PyBar>) {
        let rust_bars: Vec<Bar> = bars.iter().map(|b| b.to_rust()).collect();
        let chart = self.take_inner().bars(&rust_bars);
        self.put_inner(chart);
    }

    /// Accept Python list of floats
    fn overlay(&mut self, name: &str, values: Vec<f64>, color: &str) {
        let chart = self.take_inner().overlay(name, values, color);
        self.put_inner(chart);
    }

    /// Return list of floats
    fn get_prices(&self) -> Vec<f64> {
        self.inner.as_ref()
            .map(|c| c.get_prices())
            .unwrap_or_default()
    }
}
```

### Working with Tuples

```rust
#[pymethods]
impl PyChart {
    /// Accept tuple as two parameters
    fn trend_line(&mut self, start: (f64, f64), end: (f64, f64)) {
        let primitive = PrimitiveConfig::trend_line(start, end);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Accept list of tuples
    fn polyline(&mut self, points: Vec<(f64, f64)>) {
        let primitive = PrimitiveConfig::polyline(points);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }
}
```

### Optional Parameters

```rust
#[pymethods]
impl PyChart {
    #[pyo3(signature = (bar_index, price, label=None))]
    fn buy_signal(&mut self, bar_index: usize, price: f64, label: Option<String>) {
        let mut signal = SignalConfig::buy(bar_index, price);
        if let Some(l) = label {
            signal = signal.with_label(&l);
        }
        let chart = self.take_inner().signal(signal);
        self.put_inner(chart);
    }
}
```

Python usage:
```python
chart.buy_signal(10, 105.0)           # No label
chart.buy_signal(15, 108.0, "Entry")  # With label
```

### Default Values

```rust
#[pymethods]
impl PyBar {
    #[new]
    #[pyo3(signature = (timestamp, open, high, low, close, volume=0.0))]
    fn new(timestamp: i64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
        // volume defaults to 0.0 if not provided
        Self {
            inner: Bar { timestamp, open, high, low, close, volume },
        }
    }
}
```

Python usage:
```python
bar = Bar(ts, o, h, l, c)         # volume = 0.0
bar = Bar(ts, o, h, l, c, 1000.0) # volume = 1000.0
```

### Enums

```rust
// Python-visible enum
#[pyclass]
#[derive(Clone, Copy)]
pub enum PySeriesType {
    Candlestick,
    Line,
    Area,
    HeikinAshi,
}

#[pymethods]
impl PyChart {
    fn set_series_type(&mut self, series_type: PySeriesType) {
        // ...
    }
}
```

Python usage:
```python
from zengeld_canvas import SeriesType

chart.set_series_type(SeriesType.Candlestick)
```

---

## See Also

- [Main CONTRIBUTING.md](../../CONTRIBUTING.md)
- [ARCHITECTURE.md](../../ARCHITECTURE.md)
