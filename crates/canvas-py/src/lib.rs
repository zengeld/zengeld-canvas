//! Python bindings for zengeld-canvas
//!
//! This crate provides Python bindings via PyO3 for the zengeld-canvas
//! chart rendering library.

use pyo3::prelude::*;

// Use fully qualified paths to avoid name conflicts with the pymodule
use ::zengeld_canvas::Bar;
use ::zengeld_canvas::ChartConfig;
use ::zengeld_canvas::Theme;
use ::zengeld_canvas::Viewport;

/// A single OHLCV bar
#[pyclass(name = "Bar")]
#[derive(Clone)]
pub struct PyBar {
    inner: Bar,
}

#[pymethods]
impl PyBar {
    #[new]
    #[pyo3(signature = (timestamp, open, high, low, close, volume=0.0))]
    fn new(timestamp: i64, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
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

    #[getter]
    fn timestamp(&self) -> i64 {
        self.inner.timestamp
    }

    #[getter]
    fn open(&self) -> f64 {
        self.inner.open
    }

    #[getter]
    fn high(&self) -> f64 {
        self.inner.high
    }

    #[getter]
    fn low(&self) -> f64 {
        self.inner.low
    }

    #[getter]
    fn close(&self) -> f64 {
        self.inner.close
    }

    #[getter]
    fn volume(&self) -> f64 {
        self.inner.volume
    }

    fn is_bullish(&self) -> bool {
        self.inner.close > self.inner.open
    }

    fn __repr__(&self) -> String {
        format!(
            "Bar(timestamp={}, o={}, h={}, l={}, c={}, v={})",
            self.inner.timestamp,
            self.inner.open,
            self.inner.high,
            self.inner.low,
            self.inner.close,
            self.inner.volume
        )
    }
}

/// Chart viewport managing visible area
#[pyclass(name = "Viewport")]
pub struct PyViewport {
    inner: Viewport,
}

#[pymethods]
impl PyViewport {
    #[new]
    #[pyo3(signature = (width, height))]
    fn new(width: f64, height: f64) -> Self {
        Self {
            inner: Viewport::new(width, height),
        }
    }

    #[getter]
    fn chart_width(&self) -> f64 {
        self.inner.chart_width()
    }

    #[getter]
    fn chart_height(&self) -> f64 {
        self.inner.chart_height
    }

    #[getter]
    fn bar_width(&self) -> f64 {
        self.inner.bar_width()
    }

    /// Set chart dimensions
    fn set_size(&mut self, width: f64, height: f64) {
        self.inner.set_size(width, height);
    }

    /// Set number of bars
    fn set_bar_count(&mut self, count: usize) {
        self.inner.set_bar_count(count);
    }

    /// Scroll to end of chart
    fn scroll_to_end(&mut self) {
        self.inner.scroll_to_end();
    }

    /// Scroll to beginning
    fn scroll_to_start(&mut self) {
        self.inner.scroll_to_start();
    }
}

/// Chart theme (colors)
#[pyclass(name = "Theme")]
pub struct PyTheme {
    inner: Theme,
}

#[pymethods]
impl PyTheme {
    #[staticmethod]
    fn dark() -> Self {
        Self {
            inner: Theme::dark(),
        }
    }

    #[staticmethod]
    fn light() -> Self {
        Self {
            inner: Theme::light(),
        }
    }

    #[getter]
    fn bg_color(&self) -> &str {
        self.inner.bg_color
    }

    #[getter]
    fn text_color(&self) -> &str {
        self.inner.text_color
    }

    #[getter]
    fn grid_color(&self) -> &str {
        self.inner.grid_color
    }

    #[getter]
    fn candle_up(&self) -> &str {
        self.inner.candle_up
    }

    #[getter]
    fn candle_down(&self) -> &str {
        self.inner.candle_down
    }
}

/// Chart configuration
#[pyclass(name = "ChartConfig")]
pub struct PyChartConfig {
    #[allow(dead_code)]
    inner: ChartConfig,
}

#[pymethods]
impl PyChartConfig {
    #[new]
    fn new() -> Self {
        Self {
            inner: ChartConfig::default(),
        }
    }
}

/// Get library version
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Python module definition
#[pymodule]
fn zengeld_canvas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<PyBar>()?;
    m.add_class::<PyViewport>()?;
    m.add_class::<PyTheme>()?;
    m.add_class::<PyChartConfig>()?;
    Ok(())
}
