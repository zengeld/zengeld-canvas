# Contributing to canvas-wasm

This guide covers how to add and modify WebAssembly bindings for JavaScript/TypeScript.

## Table of Contents

- [Adding a New JS Class](#adding-a-new-js-class)
- [Naming Conventions](#naming-conventions)
- [Common Patterns](#common-patterns)
- [TypeScript Integration](#typescript-integration)

---

## Adding a New JS Class

### Step 1: Define the Wrapper Struct

```rust
use wasm_bindgen::prelude::*;
use zengeld_canvas::SomeRustType;

#[wasm_bindgen]
pub struct MyClass {
    inner: SomeRustType,
}
```

### Step 2: Implement Constructor

```rust
#[wasm_bindgen]
impl MyClass {
    /// Constructor - becomes `new MyClass()` in JS
    #[wasm_bindgen(constructor)]
    pub fn new(param: f64) -> Self {
        Self {
            inner: SomeRustType::new(param),
        }
    }
}
```

### Step 3: Add Methods

```rust
#[wasm_bindgen]
impl MyClass {
    // Getter - becomes `obj.value` in JS
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> f64 {
        self.inner.value()
    }

    // Setter - becomes `obj.value = x` in JS
    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: f64) {
        self.inner.set_value(value);
    }

    // Method with JS name override
    #[wasm_bindgen(js_name = calculateTotal)]
    pub fn calculate_total(&self, items: &[f64]) -> f64 {
        self.inner.calculate_total(items)
    }

    // Method returning Result (becomes throwing in JS)
    #[wasm_bindgen(js_name = parseConfig)]
    pub fn parse_config(&mut self, json: &str) -> Result<(), JsError> {
        self.inner.parse_config(json)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}
```

### Step 4: Static Methods

```rust
#[wasm_bindgen]
impl MyClass {
    /// Static method - becomes `MyClass.fromJson()`
    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(json: &str) -> Option<MyClass> {
        SomeRustType::from_json(json).map(|inner| Self { inner })
    }

    /// Static factory with different name
    #[wasm_bindgen(static_method_of = MyClass, js_name = createDefault)]
    pub fn create_default() -> MyClass {
        Self {
            inner: SomeRustType::default(),
        }
    }
}
```

### Full Example

See how `Chart` is implemented in `src/lib.rs`:

```rust
// src/lib.rs (lines ~100-200)

#[wasm_bindgen]
pub struct Chart {
    inner: Option<RustChart>,
}

impl Chart {
    fn take_inner(&mut self) -> RustChart {
        self.inner.take().expect("Chart already consumed")
    }
    fn put_inner(&mut self, chart: RustChart) {
        self.inner = Some(chart);
    }
}

#[wasm_bindgen]
impl Chart {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            inner: Some(RustChart::new(width, height)),
        }
    }

    // Builder methods use take/put pattern
    #[wasm_bindgen(js_name = dpr)]
    pub fn dpr(&mut self, dpr: f64) {
        let chart = self.take_inner().dpr(dpr);
        self.put_inner(chart);
    }

    // Method with array parameter
    #[wasm_bindgen(js_name = bars)]
    pub fn bars(&mut self, bars: Vec<Bar>) {
        let rust_bars: Vec<CoreBar> = bars.iter().map(|b| b.to_rust()).collect();
        let chart = self.take_inner().bars(&rust_bars);
        self.put_inner(chart);
    }

    // Final render method
    #[wasm_bindgen(js_name = renderSvg)]
    pub fn render_svg(&self) -> String {
        self.inner
            .as_ref()
            .map(|c| c.render_svg())
            .unwrap_or_default()
    }
}
```

---

## Naming Conventions

All public APIs use **camelCase** for JavaScript:

| Rust | JS | Attribute |
|------|-----|-----------|
| `bar_to_x()` | `barToX()` | `#[wasm_bindgen(js_name = barToX)]` |
| `set_bar_count()` | `setBarCount()` | `#[wasm_bindgen(js_name = setBarCount)]` |
| `render_svg()` | `renderSvg()` | `#[wasm_bindgen(js_name = renderSvg)]` |
| `is_bullish()` | `isBullish()` | `#[wasm_bindgen(js_name = isBullish)]` |

**Properties vs Methods:**

```rust
// Property (getter/setter) → JS: obj.width = 100
#[wasm_bindgen(getter)]
pub fn width(&self) -> f64 { ... }

#[wasm_bindgen(setter)]
pub fn set_width(&mut self, w: f64) { ... }

// Method → JS: obj.calculateWidth()
#[wasm_bindgen(js_name = calculateWidth)]
pub fn calculate_width(&self) -> f64 { ... }
```

---

## Common Patterns

### Working with Arrays

```rust
use js_sys::{Array, Float64Array};

// Accept JS array
#[wasm_bindgen]
pub fn process_values(&mut self, values: Vec<f64>) {
    // Vec<f64> automatically converts from JS array
}

// Return JS array
#[wasm_bindgen]
pub fn get_values(&self) -> Vec<f64> {
    self.inner.get_values()
}

// For typed arrays (better performance)
#[wasm_bindgen]
pub fn process_typed(&mut self, values: Float64Array) {
    let vec: Vec<f64> = values.to_vec();
    // ...
}
```

### Working with Tuples

```rust
// Tuples become arrays in JS
#[wasm_bindgen]
pub fn trend_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
    // In JS: chart.trendLine(0, 100, 10, 110)
}

// Alternative: use array parameter
#[wasm_bindgen(js_name = trendLinePoints)]
pub fn trend_line_points(&mut self, p1: Vec<f64>, p2: Vec<f64>) {
    // In JS: chart.trendLinePoints([0, 100], [10, 110])
    let (x1, y1) = (p1[0], p1[1]);
    let (x2, y2) = (p2[0], p2[1]);
}
```

### Optional Parameters

```rust
#[wasm_bindgen]
pub fn add_signal(
    &mut self,
    bar_index: usize,
    price: f64,
    label: Option<String>
) {
    // label is undefined in JS if not provided
}
```

### Error Handling

```rust
use wasm_bindgen::JsError;

#[wasm_bindgen]
pub fn parse(&mut self, json: &str) -> Result<(), JsError> {
    self.inner.parse(json)
        .map_err(|e| JsError::new(&format!("Parse error: {}", e)))
}
```

### Console Logging (Debug)

```rust
use web_sys::console;

#[wasm_bindgen]
pub fn debug_info(&self) {
    console::log_1(&format!("Chart size: {}x{}", self.width, self.height).into());
}
```

---

## TypeScript Integration

wasm-pack generates `.d.ts` files automatically from your Rust types.

---

## See Also

- [Main CONTRIBUTING.md](../../CONTRIBUTING.md)
- [ARCHITECTURE.md](../../ARCHITECTURE.md)
