//! WebAssembly bindings for zengeld-canvas
//!
//! This crate provides WASM bindings via wasm-bindgen for the zengeld-canvas
//! chart rendering library.

use wasm_bindgen::prelude::*;
use zengeld_canvas::{Bar, ChartConfig, Theme, Viewport};

/// A single OHLCV bar
#[wasm_bindgen]
pub struct JsBar {
    inner: Bar,
}

#[wasm_bindgen]
impl JsBar {
    #[wasm_bindgen(constructor)]
    pub fn new(timestamp: i64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
        Self {
            inner: Bar {
                timestamp,
                open,
                high,
                low,
                close,
                volume,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn timestamp(&self) -> i64 {
        self.inner.timestamp
    }

    #[wasm_bindgen(getter)]
    pub fn open(&self) -> f64 {
        self.inner.open
    }

    #[wasm_bindgen(getter)]
    pub fn high(&self) -> f64 {
        self.inner.high
    }

    #[wasm_bindgen(getter)]
    pub fn low(&self) -> f64 {
        self.inner.low
    }

    #[wasm_bindgen(getter)]
    pub fn close(&self) -> f64 {
        self.inner.close
    }

    #[wasm_bindgen(getter)]
    pub fn volume(&self) -> f64 {
        self.inner.volume
    }

    #[wasm_bindgen(js_name = isBullish)]
    pub fn is_bullish(&self) -> bool {
        self.inner.close > self.inner.open
    }
}

/// Chart viewport managing visible area
#[wasm_bindgen]
pub struct JsViewport {
    inner: Viewport,
}

#[wasm_bindgen]
impl JsViewport {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            inner: Viewport::new(width, height),
        }
    }

    #[wasm_bindgen(getter, js_name = chartWidth)]
    pub fn chart_width(&self) -> f64 {
        self.inner.chart_width()
    }

    #[wasm_bindgen(getter, js_name = chartHeight)]
    pub fn chart_height(&self) -> f64 {
        self.inner.chart_height
    }

    #[wasm_bindgen(getter, js_name = barWidth)]
    pub fn bar_width(&self) -> f64 {
        self.inner.bar_width()
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.inner.set_size(width, height);
    }

    #[wasm_bindgen(js_name = setBarCount)]
    pub fn set_bar_count(&mut self, count: usize) {
        self.inner.set_bar_count(count);
    }

    #[wasm_bindgen(js_name = scrollToEnd)]
    pub fn scroll_to_end(&mut self) {
        self.inner.scroll_to_end();
    }

    #[wasm_bindgen(js_name = scrollToStart)]
    pub fn scroll_to_start(&mut self) {
        self.inner.scroll_to_start();
    }
}

/// Chart theme (colors)
#[wasm_bindgen]
pub struct JsTheme {
    inner: Theme,
}

#[wasm_bindgen]
impl JsTheme {
    #[wasm_bindgen]
    pub fn dark() -> Self {
        Self {
            inner: Theme::dark(),
        }
    }

    #[wasm_bindgen]
    pub fn light() -> Self {
        Self {
            inner: Theme::light(),
        }
    }

    #[wasm_bindgen(getter, js_name = bgColor)]
    pub fn bg_color(&self) -> String {
        self.inner.bg_color.to_string()
    }

    #[wasm_bindgen(getter, js_name = textColor)]
    pub fn text_color(&self) -> String {
        self.inner.text_color.to_string()
    }

    #[wasm_bindgen(getter, js_name = gridColor)]
    pub fn grid_color(&self) -> String {
        self.inner.grid_color.to_string()
    }

    #[wasm_bindgen(getter, js_name = candleUp)]
    pub fn candle_up(&self) -> String {
        self.inner.candle_up.to_string()
    }

    #[wasm_bindgen(getter, js_name = candleDown)]
    pub fn candle_down(&self) -> String {
        self.inner.candle_down.to_string()
    }
}

/// Chart configuration
#[wasm_bindgen]
pub struct JsChartConfig {
    #[allow(dead_code)]
    inner: ChartConfig,
}

#[wasm_bindgen]
impl JsChartConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: ChartConfig::default(),
        }
    }
}

impl Default for JsChartConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Get library version
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
