//! WebAssembly bindings for zengeld-canvas
//!
//! Complete JavaScript/TypeScript API for the zengeld-canvas chart rendering library.
//! Provides 1:1 mapping to Rust API.
//!
//! # Quick Start
//!
//! ```javascript
//! import init, { Chart, JsBar } from 'zengeld-canvas';
//!
//! async function main() {
//!   await init();
//!
//!   // Create bars
//!   const bars = [new JsBar(1704067200n, 100.0, 105.0, 99.0, 103.0, 1000.0)];
//!
//!   // Create chart and render
//!   const chart = new Chart(800, 600);
//!   chart.setBars(bars);
//!   chart.candlesticks();
//!   chart.sma(20, "#2196F3");
//!   const svg = chart.renderSvg();
//! }
//! ```

use wasm_bindgen::prelude::*;
use zengeld_canvas::api::{
    Chart as RustChart, ChartConfig as RustChartConfig, PrimitiveConfig, SignalConfig,
};
use zengeld_canvas::core::Bar;
use zengeld_canvas::model::Indicator;
use zengeld_canvas::{Theme, Viewport};

// =============================================================================
// JsBar - OHLCV data point
// =============================================================================

/// A single OHLCV bar representing one time period of price data.
#[wasm_bindgen]
pub struct JsBar {
    inner: Bar,
}

#[wasm_bindgen]
impl JsBar {
    /// Create a new OHLCV bar.
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

// =============================================================================
// Chart - Main chart builder API
// =============================================================================

/// High-level chart builder for creating financial charts.
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
    /// Create a new chart with specified dimensions.
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            inner: Some(RustChart::new(width, height)),
        }
    }

    // =========================================================================
    // Configuration
    // =========================================================================

    /// Set device pixel ratio for high-DPI displays
    #[wasm_bindgen(js_name = setDpr)]
    pub fn set_dpr(&mut self, dpr: f64) {
        let chart = self.take_inner().dpr(dpr);
        self.put_inner(chart);
    }

    /// Set OHLCV bar data
    #[wasm_bindgen(js_name = setBars)]
    pub fn set_bars(&mut self, bars: Vec<JsBar>) {
        let rust_bars: Vec<Bar> = bars.iter().map(|b| b.inner).collect();
        let chart = self.take_inner().bars(&rust_bars);
        self.put_inner(chart);
    }

    // =========================================================================
    // Series Types
    // =========================================================================

    /// Candlestick chart (default)
    #[wasm_bindgen]
    pub fn candlesticks(&mut self) {
        let chart = self.take_inner().candlesticks();
        self.put_inner(chart);
    }

    /// Line chart (close prices)
    #[wasm_bindgen]
    pub fn line(&mut self) {
        let chart = self.take_inner().line();
        self.put_inner(chart);
    }

    /// Area chart (filled)
    #[wasm_bindgen]
    pub fn area(&mut self) {
        let chart = self.take_inner().area();
        self.put_inner(chart);
    }

    // =========================================================================
    // Theme & Styling
    // =========================================================================

    /// Set up/down colors
    #[wasm_bindgen(js_name = setColors)]
    pub fn set_colors(&mut self, up: &str, down: &str) {
        let chart = self.take_inner().colors(up, down);
        self.put_inner(chart);
    }

    /// Set background color
    #[wasm_bindgen(js_name = setBackground)]
    pub fn set_background(&mut self, color: &str) {
        let chart = self.take_inner().background(color);
        self.put_inner(chart);
    }

    /// Enable/disable grid
    #[wasm_bindgen(js_name = setGrid)]
    pub fn set_grid(&mut self, show: bool) {
        let chart = self.take_inner().grid(show);
        self.put_inner(chart);
    }

    /// Apply dark theme
    #[wasm_bindgen(js_name = darkTheme)]
    pub fn dark_theme(&mut self) {
        let chart = self
            .take_inner()
            .background("#131722")
            .colors("#26a69a", "#ef5350");
        self.put_inner(chart);
    }

    /// Apply light theme
    #[wasm_bindgen(js_name = lightTheme)]
    pub fn light_theme(&mut self) {
        let chart = self
            .take_inner()
            .background("#ffffff")
            .colors("#26a69a", "#ef5350");
        self.put_inner(chart);
    }

    // =========================================================================
    // Moving Average Indicators (9 types)
    // =========================================================================

    /// Simple Moving Average
    #[wasm_bindgen]
    pub fn sma(&mut self, period: usize, color: &str) {
        let chart = self.take_inner().sma(period, color);
        self.put_inner(chart);
    }

    /// Exponential Moving Average
    #[wasm_bindgen]
    pub fn ema(&mut self, period: usize, color: &str) {
        let chart = self.take_inner().ema(period, color);
        self.put_inner(chart);
    }

    /// Weighted Moving Average
    #[wasm_bindgen]
    pub fn wma(&mut self, period: usize, color: &str) {
        let id = format!("wma_{}", period);
        let indicator = Indicator::wma(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Hull Moving Average
    #[wasm_bindgen]
    pub fn hma(&mut self, period: usize, color: &str) {
        let id = format!("hma_{}", period);
        let indicator = Indicator::hma(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Double Exponential Moving Average
    #[wasm_bindgen]
    pub fn dema(&mut self, period: usize, color: &str) {
        let id = format!("dema_{}", period);
        let indicator = Indicator::dema(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Triple Exponential Moving Average
    #[wasm_bindgen]
    pub fn tema(&mut self, period: usize, color: &str) {
        let id = format!("tema_{}", period);
        let indicator = Indicator::tema(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Kaufman Adaptive Moving Average
    #[wasm_bindgen]
    pub fn kama(&mut self, period: usize, color: &str) {
        let id = format!("kama_{}", period);
        let indicator = Indicator::kama(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Triangular Moving Average
    #[wasm_bindgen]
    pub fn trima(&mut self, period: usize, color: &str) {
        let id = format!("trima_{}", period);
        let indicator = Indicator::trima(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Zero Lag EMA
    #[wasm_bindgen]
    pub fn zlema(&mut self, period: usize, color: &str) {
        let id = format!("zlema_{}", period);
        let indicator = Indicator::zlema(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// McGinley Dynamic
    #[wasm_bindgen]
    pub fn mcginley(&mut self, period: usize, color: &str) {
        let id = format!("mcginley_{}", period);
        let indicator = Indicator::mcginley(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    // =========================================================================
    // Band Indicators (5 types)
    // =========================================================================

    /// Bollinger Bands
    #[wasm_bindgen]
    pub fn bollinger(&mut self, period: usize, multiplier: f64) {
        let chart = self.take_inner().bollinger(period, multiplier);
        self.put_inner(chart);
    }

    /// Bollinger Bands with filled cloud
    #[wasm_bindgen(js_name = bollingerFilled)]
    pub fn bollinger_filled(&mut self, period: usize) {
        let id = format!("bb_filled_{}", period);
        let indicator = Indicator::bollinger_filled(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Keltner Channel
    #[wasm_bindgen]
    pub fn keltner(&mut self, period: usize) {
        let id = format!("keltner_{}", period);
        let indicator = Indicator::keltner(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Donchian Channel
    #[wasm_bindgen]
    pub fn donchian(&mut self, period: usize) {
        let id = format!("donchian_{}", period);
        let indicator = Indicator::donchian(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// ATR Bands
    #[wasm_bindgen(js_name = atrBands)]
    pub fn atr_bands(&mut self, period: usize, multiplier: f64) {
        let id = format!("atr_bands_{}", period);
        let indicator = Indicator::atr_bands(&id, period as u32, multiplier);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    // =========================================================================
    // Oscillators / Momentum Indicators
    // =========================================================================

    /// Relative Strength Index
    #[wasm_bindgen]
    pub fn rsi(&mut self, period: usize) {
        let chart = self.take_inner().rsi(period);
        self.put_inner(chart);
    }

    /// MACD
    #[wasm_bindgen]
    pub fn macd(&mut self, fast: usize, slow: usize, signal: usize) {
        let chart = self.take_inner().macd(fast, slow, signal);
        self.put_inner(chart);
    }

    /// MACD with default settings (12, 26, 9)
    #[wasm_bindgen(js_name = macdDefault)]
    pub fn macd_default(&mut self) {
        let indicator = Indicator::macd_default("macd");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Stochastic Oscillator
    #[wasm_bindgen]
    pub fn stochastic(&mut self, k: usize, d: usize) {
        let id = format!("stoch_{}_{}", k, d);
        let indicator = Indicator::stochastic(&id, k as u32, d as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Stochastic RSI
    #[wasm_bindgen(js_name = stochRsi)]
    pub fn stoch_rsi(&mut self, period: usize) {
        let id = format!("stoch_rsi_{}", period);
        let indicator = Indicator::stoch_rsi(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Commodity Channel Index
    #[wasm_bindgen]
    pub fn cci(&mut self, period: usize) {
        let id = format!("cci_{}", period);
        let indicator = Indicator::cci(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Williams %R
    #[wasm_bindgen(js_name = williamsR)]
    pub fn williams_r(&mut self, period: usize) {
        let id = format!("williams_r_{}", period);
        let indicator = Indicator::williams_r(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Momentum
    #[wasm_bindgen]
    pub fn momentum(&mut self, period: usize) {
        let id = format!("momentum_{}", period);
        let indicator = Indicator::momentum(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Rate of Change
    #[wasm_bindgen]
    pub fn roc(&mut self, period: usize) {
        let id = format!("roc_{}", period);
        let indicator = Indicator::roc(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// True Strength Index
    #[wasm_bindgen]
    pub fn tsi(&mut self, r: usize, s: usize) {
        let id = format!("tsi_{}_{}", r, s);
        let indicator = Indicator::tsi(&id, r as u32, s as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Ultimate Oscillator
    #[wasm_bindgen(js_name = ultimateOscillator)]
    pub fn ultimate_oscillator(&mut self) {
        let indicator = Indicator::ultimate_oscillator("uo");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Awesome Oscillator
    #[wasm_bindgen(js_name = awesomeOscillator)]
    pub fn awesome_oscillator(&mut self) {
        let indicator = Indicator::awesome_oscillator("ao");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Accelerator Oscillator
    #[wasm_bindgen(js_name = acceleratorOscillator)]
    pub fn accelerator_oscillator(&mut self) {
        let indicator = Indicator::accelerator_oscillator("ac");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Chande Momentum Oscillator
    #[wasm_bindgen]
    pub fn cmo(&mut self, period: usize) {
        let id = format!("cmo_{}", period);
        let indicator = Indicator::cmo(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Detrended Price Oscillator
    #[wasm_bindgen]
    pub fn dpo(&mut self, period: usize) {
        let id = format!("dpo_{}", period);
        let indicator = Indicator::dpo(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Know Sure Thing
    #[wasm_bindgen]
    pub fn kst(&mut self) {
        let indicator = Indicator::kst("kst");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Relative Vigor Index
    #[wasm_bindgen]
    pub fn rvi(&mut self, period: usize) {
        let id = format!("rvi_{}", period);
        let indicator = Indicator::rvi(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    // =========================================================================
    // Volatility Indicators
    // =========================================================================

    /// Average True Range
    #[wasm_bindgen]
    pub fn atr(&mut self, period: usize) {
        let id = format!("atr_{}", period);
        let indicator = Indicator::atr(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Standard Deviation
    #[wasm_bindgen]
    pub fn stddev(&mut self, period: usize) {
        let id = format!("stddev_{}", period);
        let indicator = Indicator::stddev(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Historical Volatility
    #[wasm_bindgen(js_name = historicalVolatility)]
    pub fn historical_volatility(&mut self, period: usize) {
        let id = format!("hv_{}", period);
        let indicator = Indicator::historical_volatility(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Choppiness Index
    #[wasm_bindgen]
    pub fn choppiness(&mut self, period: usize) {
        let id = format!("chop_{}", period);
        let indicator = Indicator::choppiness(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Mass Index
    #[wasm_bindgen(js_name = massIndex)]
    pub fn mass_index(&mut self) {
        let indicator = Indicator::mass_index("mass");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Ulcer Index
    #[wasm_bindgen(js_name = ulcerIndex)]
    pub fn ulcer_index(&mut self, period: usize) {
        let id = format!("ulcer_{}", period);
        let indicator = Indicator::ulcer_index(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    // =========================================================================
    // Trend Indicators
    // =========================================================================

    /// Average Directional Index
    #[wasm_bindgen]
    pub fn adx(&mut self, period: usize) {
        let id = format!("adx_{}", period);
        let indicator = Indicator::adx(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Aroon
    #[wasm_bindgen]
    pub fn aroon(&mut self, period: usize) {
        let id = format!("aroon_{}", period);
        let indicator = Indicator::aroon(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Aroon Oscillator
    #[wasm_bindgen(js_name = aroonOscillator)]
    pub fn aroon_oscillator(&mut self, period: usize) {
        let id = format!("aroon_osc_{}", period);
        let indicator = Indicator::aroon_oscillator(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Vortex Indicator
    #[wasm_bindgen]
    pub fn vortex(&mut self, period: usize) {
        let id = format!("vortex_{}", period);
        let indicator = Indicator::vortex(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Linear Regression
    #[wasm_bindgen(js_name = linearRegression)]
    pub fn linear_regression(&mut self, period: usize, color: &str) {
        let id = format!("linreg_{}", period);
        let indicator = Indicator::linear_regression(&id, period as u32, color);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Linear Regression Slope
    #[wasm_bindgen(js_name = linearRegressionSlope)]
    pub fn linear_regression_slope(&mut self, period: usize) {
        let id = format!("linreg_slope_{}", period);
        let indicator = Indicator::linear_regression_slope(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// ZigZag
    #[wasm_bindgen]
    pub fn zigzag(&mut self, deviation: f64) {
        let indicator = Indicator::zigzag("zigzag", deviation);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// TRIX
    #[wasm_bindgen]
    pub fn trix(&mut self, period: usize) {
        let id = format!("trix_{}", period);
        let indicator = Indicator::trix(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Chande Kroll Stop
    #[wasm_bindgen(js_name = chandeKrollStop)]
    pub fn chande_kroll_stop(&mut self) {
        let indicator = Indicator::chande_kroll_stop("cks");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Parabolic SAR
    #[wasm_bindgen]
    pub fn psar(&mut self) {
        let indicator = Indicator::psar("psar");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Supertrend
    #[wasm_bindgen]
    pub fn supertrend(&mut self, period: usize, multiplier: f64) {
        let id = format!("supertrend_{}", period);
        let indicator = Indicator::supertrend(&id, period as u32, multiplier);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Ichimoku Cloud
    #[wasm_bindgen]
    pub fn ichimoku(&mut self) {
        let indicator = Indicator::ichimoku("ichimoku");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    // =========================================================================
    // Volume Indicators
    // =========================================================================

    /// Volume (subpane)
    #[wasm_bindgen]
    pub fn volume(&mut self) {
        let chart = self.take_inner().volume();
        self.put_inner(chart);
    }

    /// On Balance Volume
    #[wasm_bindgen]
    pub fn obv(&mut self) {
        let indicator = Indicator::obv("obv");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Accumulation/Distribution Line
    #[wasm_bindgen(js_name = adLine)]
    pub fn ad_line(&mut self) {
        let indicator = Indicator::ad_line("ad");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Chaikin Money Flow
    #[wasm_bindgen]
    pub fn cmf(&mut self, period: usize) {
        let id = format!("cmf_{}", period);
        let indicator = Indicator::cmf(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Chaikin Oscillator
    #[wasm_bindgen(js_name = chaikinOscillator)]
    pub fn chaikin_oscillator(&mut self) {
        let indicator = Indicator::chaikin_oscillator("cho");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Volume Price Trend
    #[wasm_bindgen]
    pub fn vpt(&mut self) {
        let indicator = Indicator::vpt("vpt");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Force Index
    #[wasm_bindgen(js_name = forceIndex)]
    pub fn force_index(&mut self, period: usize) {
        let id = format!("fi_{}", period);
        let indicator = Indicator::force_index(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Ease of Movement
    #[wasm_bindgen]
    pub fn eom(&mut self, period: usize) {
        let id = format!("eom_{}", period);
        let indicator = Indicator::eom(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Negative Volume Index
    #[wasm_bindgen]
    pub fn nvi(&mut self) {
        let indicator = Indicator::nvi("nvi");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Positive Volume Index
    #[wasm_bindgen]
    pub fn pvi(&mut self) {
        let indicator = Indicator::pvi("pvi");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Money Flow Index
    #[wasm_bindgen]
    pub fn mfi(&mut self, period: usize) {
        let id = format!("mfi_{}", period);
        let indicator = Indicator::mfi(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// VWAP
    #[wasm_bindgen]
    pub fn vwap(&mut self) {
        let indicator = Indicator::vwap("vwap");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    // =========================================================================
    // Specialized Indicators
    // =========================================================================

    /// Elder Ray
    #[wasm_bindgen(js_name = elderRay)]
    pub fn elder_ray(&mut self, period: usize) {
        let id = format!("elder_ray_{}", period);
        let indicator = Indicator::elder_ray(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Balance of Power
    #[wasm_bindgen(js_name = balanceOfPower)]
    pub fn balance_of_power(&mut self) {
        let indicator = Indicator::balance_of_power("bop");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Connors RSI
    #[wasm_bindgen(js_name = connorsRsi)]
    pub fn connors_rsi(&mut self) {
        let indicator = Indicator::connors_rsi("crsi");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Coppock Curve
    #[wasm_bindgen(js_name = coppockCurve)]
    pub fn coppock_curve(&mut self) {
        let indicator = Indicator::coppock_curve("coppock");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Fisher Transform
    #[wasm_bindgen(js_name = fisherTransform)]
    pub fn fisher_transform(&mut self, period: usize) {
        let id = format!("fisher_{}", period);
        let indicator = Indicator::fisher_transform(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// SMI Ergodic
    #[wasm_bindgen(js_name = smiErgodic)]
    pub fn smi_ergodic(&mut self) {
        let indicator = Indicator::smi_ergodic("smi");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Schaff Trend Cycle
    #[wasm_bindgen(js_name = schaffTrendCycle)]
    pub fn schaff_trend_cycle(&mut self) {
        let indicator = Indicator::schaff_trend_cycle("stc");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Pretty Good Oscillator
    #[wasm_bindgen]
    pub fn pgo(&mut self, period: usize) {
        let id = format!("pgo_{}", period);
        let indicator = Indicator::pgo(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// QStick
    #[wasm_bindgen]
    pub fn qstick(&mut self, period: usize) {
        let id = format!("qstick_{}", period);
        let indicator = Indicator::qstick(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Pivot Points
    #[wasm_bindgen(js_name = pivotPoints)]
    pub fn pivot_points(&mut self) {
        let indicator = Indicator::pivot_points("pp");
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Envelopes
    #[wasm_bindgen]
    pub fn envelopes(&mut self, period: usize, percent: f64) {
        let id = format!("env_{}", period);
        let indicator = Indicator::envelopes(&id, period as u32, percent);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Price Channel
    #[wasm_bindgen(js_name = priceChannel)]
    pub fn price_channel(&mut self, period: usize) {
        let id = format!("pc_{}", period);
        let indicator = Indicator::price_channel(&id, period as u32);
        let chart = self.take_inner().indicator(indicator);
        self.put_inner(chart);
    }

    /// Custom overlay with pre-calculated values
    #[wasm_bindgen(js_name = addOverlay)]
    pub fn add_overlay(&mut self, name: &str, values: Vec<f64>, color: &str) {
        let chart = self.take_inner().overlay(name, values, color);
        self.put_inner(chart);
    }

    // =========================================================================
    // Signals (7 types)
    // =========================================================================

    /// Buy signal
    #[wasm_bindgen(js_name = buySignal)]
    pub fn buy_signal(&mut self, bar_index: usize, price: f64, label: Option<String>) {
        let mut signal = SignalConfig::buy(bar_index, price);
        if let Some(l) = label {
            signal = signal.with_label(&l);
        }
        let chart = self.take_inner().signal(signal);
        self.put_inner(chart);
    }

    /// Sell signal
    #[wasm_bindgen(js_name = sellSignal)]
    pub fn sell_signal(&mut self, bar_index: usize, price: f64, label: Option<String>) {
        let mut signal = SignalConfig::sell(bar_index, price);
        if let Some(l) = label {
            signal = signal.with_label(&l);
        }
        let chart = self.take_inner().signal(signal);
        self.put_inner(chart);
    }

    /// Entry signal
    #[wasm_bindgen(js_name = entrySignal)]
    pub fn entry_signal(&mut self, bar_index: usize, price: f64, label: Option<String>) {
        let mut signal = SignalConfig::entry(bar_index, price);
        if let Some(l) = label {
            signal = signal.with_label(&l);
        }
        let chart = self.take_inner().signal(signal);
        self.put_inner(chart);
    }

    /// Exit signal
    #[wasm_bindgen(js_name = exitSignal)]
    pub fn exit_signal(&mut self, bar_index: usize, price: f64, label: Option<String>) {
        let mut signal = SignalConfig::exit(bar_index, price);
        if let Some(l) = label {
            signal = signal.with_label(&l);
        }
        let chart = self.take_inner().signal(signal);
        self.put_inner(chart);
    }

    /// Take profit signal
    #[wasm_bindgen(js_name = takeProfitSignal)]
    pub fn take_profit_signal(&mut self, bar_index: usize, price: f64, label: Option<String>) {
        let mut signal = SignalConfig::take_profit(bar_index, price);
        if let Some(l) = label {
            signal = signal.with_label(&l);
        }
        let chart = self.take_inner().signal(signal);
        self.put_inner(chart);
    }

    /// Stop loss signal
    #[wasm_bindgen(js_name = stopLossSignal)]
    pub fn stop_loss_signal(&mut self, bar_index: usize, price: f64, label: Option<String>) {
        let mut signal = SignalConfig::stop_loss(bar_index, price);
        if let Some(l) = label {
            signal = signal.with_label(&l);
        }
        let chart = self.take_inner().signal(signal);
        self.put_inner(chart);
    }

    /// Custom signal
    #[wasm_bindgen(js_name = customSignal)]
    pub fn custom_signal(&mut self, bar_index: usize, price: f64, label: &str) {
        let signal = SignalConfig::custom(bar_index, price, label);
        let chart = self.take_inner().signal(signal);
        self.put_inner(chart);
    }

    // =========================================================================
    // Lines (9 primitives)
    // =========================================================================

    /// Horizontal line at price level
    #[wasm_bindgen(js_name = horizontalLine)]
    pub fn horizontal_line(&mut self, price: f64) {
        let primitive = PrimitiveConfig::horizontal_line(price);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Vertical line at bar index
    #[wasm_bindgen(js_name = verticalLine)]
    pub fn vertical_line(&mut self, bar_index: f64) {
        let primitive = PrimitiveConfig::vertical_line(bar_index);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Trend line between two points
    #[wasm_bindgen(js_name = trendLine)]
    pub fn trend_line(&mut self, start_x: f64, start_y: f64, end_x: f64, end_y: f64) {
        let primitive = PrimitiveConfig::trend_line((start_x, start_y), (end_x, end_y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Ray from p1 through p2
    #[wasm_bindgen]
    pub fn ray(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::ray((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Extended line (infinite in both directions)
    #[wasm_bindgen(js_name = extendedLine)]
    pub fn extended_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::extended_line((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Info line with measurements
    #[wasm_bindgen(js_name = infoLine)]
    pub fn info_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::info_line((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Trend angle
    #[wasm_bindgen(js_name = trendAngle)]
    pub fn trend_angle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::trend_angle((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Horizontal ray
    #[wasm_bindgen(js_name = horizontalRay)]
    pub fn horizontal_ray(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::horizontal_ray((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Cross line at position
    #[wasm_bindgen(js_name = crossLine)]
    pub fn cross_line(&mut self, x: f64, y: f64) {
        let primitive = PrimitiveConfig::cross_line((x, y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Channels (4 primitives)
    // =========================================================================

    /// Parallel channel
    #[wasm_bindgen(js_name = parallelChannel)]
    pub fn parallel_channel(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::parallel_channel((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Regression trend channel
    #[wasm_bindgen(js_name = regressionTrend)]
    pub fn regression_trend(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::regression_trend((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Flat top/bottom channel
    #[wasm_bindgen(js_name = flatTopBottom)]
    pub fn flat_top_bottom(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::flat_top_bottom((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Disjoint channel
    #[wasm_bindgen(js_name = disjointChannel)]
    pub fn disjoint_channel(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::disjoint_channel(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Shapes (10 primitives)
    // =========================================================================

    /// Rectangle
    #[wasm_bindgen]
    pub fn rectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::rectangle((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Circle
    #[wasm_bindgen]
    pub fn circle(&mut self, cx: f64, cy: f64, ex: f64, ey: f64) {
        let primitive = PrimitiveConfig::circle((cx, cy), (ex, ey));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Ellipse
    #[wasm_bindgen]
    pub fn ellipse(&mut self, cx: f64, cy: f64, ex: f64, ey: f64) {
        let primitive = PrimitiveConfig::ellipse((cx, cy), (ex, ey));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Triangle
    #[wasm_bindgen]
    pub fn triangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::triangle((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Arc
    #[wasm_bindgen]
    pub fn arc(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::arc((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Polyline
    #[wasm_bindgen]
    pub fn polyline(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::polyline(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Path (closed polygon)
    #[wasm_bindgen]
    pub fn path(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::path(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Rotated rectangle
    #[wasm_bindgen(js_name = rotatedRectangle)]
    pub fn rotated_rectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::rotated_rectangle((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Curve
    #[wasm_bindgen]
    pub fn curve(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::curve(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Double curve
    #[wasm_bindgen(js_name = doubleCurve)]
    pub fn double_curve(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::double_curve(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Fibonacci (11 primitives)
    // =========================================================================

    /// Fibonacci retracement
    #[wasm_bindgen(js_name = fibRetracement)]
    pub fn fib_retracement(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fib_retracement((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci extension
    #[wasm_bindgen(js_name = fibExtension)]
    pub fn fib_extension(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::fib_extension((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci channel
    #[wasm_bindgen(js_name = fibChannel)]
    pub fn fib_channel(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::fib_channel((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci time zones
    #[wasm_bindgen(js_name = fibTimeZones)]
    pub fn fib_time_zones(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fib_time_zones((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci speed/resistance
    #[wasm_bindgen(js_name = fibSpeedResistance)]
    pub fn fib_speed_resistance(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fib_speed_resistance((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci trend time
    #[wasm_bindgen(js_name = fibTrendTime)]
    pub fn fib_trend_time(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::fib_trend_time((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci circles
    #[wasm_bindgen(js_name = fibCircles)]
    pub fn fib_circles(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fib_circles((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci spiral
    #[wasm_bindgen(js_name = fibSpiral)]
    pub fn fib_spiral(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fib_spiral((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci arcs
    #[wasm_bindgen(js_name = fibArcs)]
    pub fn fib_arcs(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fib_arcs((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci wedge
    #[wasm_bindgen(js_name = fibWedge)]
    pub fn fib_wedge(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fib_wedge((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fibonacci fan
    #[wasm_bindgen(js_name = fibFan)]
    pub fn fib_fan(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fib_fan((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Pitchforks (4 primitives)
    // =========================================================================

    /// Pitchfork
    #[wasm_bindgen]
    pub fn pitchfork(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::pitchfork((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Schiff Pitchfork
    #[wasm_bindgen(js_name = schiffPitchfork)]
    pub fn schiff_pitchfork(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::schiff_pitchfork((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Modified Schiff Pitchfork
    #[wasm_bindgen(js_name = modifiedSchiff)]
    pub fn modified_schiff(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::modified_schiff((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Inside Pitchfork
    #[wasm_bindgen(js_name = insidePitchfork)]
    pub fn inside_pitchfork(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::inside_pitchfork((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Gann (4 primitives)
    // =========================================================================

    /// Gann Box
    #[wasm_bindgen(js_name = gannBox)]
    pub fn gann_box(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::gann_box((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Gann Square Fixed
    #[wasm_bindgen(js_name = gannSquareFixed)]
    pub fn gann_square_fixed(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::gann_square_fixed((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Gann Square
    #[wasm_bindgen(js_name = gannSquare)]
    pub fn gann_square(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::gann_square((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Gann Fan
    #[wasm_bindgen(js_name = gannFan)]
    pub fn gann_fan(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::gann_fan((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Patterns (6 primitives)
    // =========================================================================

    /// XABCD Pattern
    #[wasm_bindgen(js_name = xabcdPattern)]
    pub fn xabcd_pattern(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::xabcd_pattern(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Cypher Pattern
    #[wasm_bindgen(js_name = cypherPattern)]
    pub fn cypher_pattern(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::cypher_pattern(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Head and Shoulders
    #[wasm_bindgen(js_name = headShoulders)]
    pub fn head_shoulders(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::head_shoulders(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// ABCD Pattern
    #[wasm_bindgen(js_name = abcdPattern)]
    pub fn abcd_pattern(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::abcd_pattern(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Triangle Pattern
    #[wasm_bindgen(js_name = trianglePattern)]
    pub fn triangle_pattern(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::triangle_pattern((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Three Drives
    #[wasm_bindgen(js_name = threeDrives)]
    pub fn three_drives(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::three_drives(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Elliott Waves (5 primitives)
    // =========================================================================

    /// Elliott Impulse Wave
    #[wasm_bindgen(js_name = elliottImpulse)]
    pub fn elliott_impulse(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::elliott_impulse(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Elliott Correction Wave
    #[wasm_bindgen(js_name = elliottCorrection)]
    pub fn elliott_correction(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::elliott_correction(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Elliott Triangle
    #[wasm_bindgen(js_name = elliottTriangle)]
    pub fn elliott_triangle(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::elliott_triangle(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Elliott Double Combo
    #[wasm_bindgen(js_name = elliottDoubleCombo)]
    pub fn elliott_double_combo(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::elliott_double_combo(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Elliott Triple Combo
    #[wasm_bindgen(js_name = elliottTripleCombo)]
    pub fn elliott_triple_combo(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::elliott_triple_combo(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Arrows (4 primitives)
    // =========================================================================

    /// Arrow marker
    #[wasm_bindgen(js_name = arrowMarker)]
    pub fn arrow_marker(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::arrow_marker((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Arrow line
    #[wasm_bindgen(js_name = arrowLine)]
    pub fn arrow_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::arrow_line((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Arrow up
    #[wasm_bindgen(js_name = arrowUp)]
    pub fn arrow_up(&mut self, x: f64, y: f64) {
        let primitive = PrimitiveConfig::arrow_up((x, y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Arrow down
    #[wasm_bindgen(js_name = arrowDown)]
    pub fn arrow_down(&mut self, x: f64, y: f64) {
        let primitive = PrimitiveConfig::arrow_down((x, y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Annotations (11 primitives)
    // =========================================================================

    /// Text annotation
    #[wasm_bindgen]
    pub fn text(&mut self, x: f64, y: f64, content: &str) {
        let primitive = PrimitiveConfig::text((x, y), content);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Anchored text
    #[wasm_bindgen(js_name = anchoredText)]
    pub fn anchored_text(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, content: &str) {
        let primitive = PrimitiveConfig::anchored_text((x1, y1), (x2, y2), content);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Note
    #[wasm_bindgen]
    pub fn note(&mut self, x: f64, y: f64, content: &str) {
        let primitive = PrimitiveConfig::note((x, y), content);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Price note
    #[wasm_bindgen(js_name = priceNote)]
    pub fn price_note(&mut self, x: f64, y: f64, content: &str) {
        let primitive = PrimitiveConfig::price_note((x, y), content);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Signpost
    #[wasm_bindgen]
    pub fn signpost(&mut self, x: f64, y: f64, content: &str) {
        let primitive = PrimitiveConfig::signpost((x, y), content);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Callout
    #[wasm_bindgen]
    pub fn callout(&mut self, x: f64, y: f64, content: &str) {
        let primitive = PrimitiveConfig::callout((x, y), content);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Comment
    #[wasm_bindgen]
    pub fn comment(&mut self, x: f64, y: f64, content: &str) {
        let primitive = PrimitiveConfig::comment((x, y), content);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Price label
    #[wasm_bindgen(js_name = priceLabel)]
    pub fn price_label(&mut self, x: f64, y: f64) {
        let primitive = PrimitiveConfig::price_label((x, y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Sign
    #[wasm_bindgen]
    pub fn sign(&mut self, x: f64, y: f64) {
        let primitive = PrimitiveConfig::sign((x, y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Flag
    #[wasm_bindgen]
    pub fn flag(&mut self, x: f64, y: f64) {
        let primitive = PrimitiveConfig::flag((x, y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Table
    #[wasm_bindgen]
    pub fn table(&mut self, x: f64, y: f64) {
        let primitive = PrimitiveConfig::table((x, y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Cycles (3 primitives)
    // =========================================================================

    /// Cycle lines
    #[wasm_bindgen(js_name = cycleLines)]
    pub fn cycle_lines(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::cycle_lines((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Time cycles
    #[wasm_bindgen(js_name = timeCycles)]
    pub fn time_cycles(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::time_cycles((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Sine wave
    #[wasm_bindgen(js_name = sineWave)]
    pub fn sine_wave(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::sine_wave((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Projections (6 primitives)
    // =========================================================================

    /// Long position
    #[wasm_bindgen(js_name = longPosition)]
    pub fn long_position(&mut self, ex: f64, ey: f64, tpx: f64, tpy: f64, slx: f64, sly: f64) {
        let primitive = PrimitiveConfig::long_position((ex, ey), (tpx, tpy), (slx, sly));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Short position
    #[wasm_bindgen(js_name = shortPosition)]
    pub fn short_position(&mut self, ex: f64, ey: f64, tpx: f64, tpy: f64, slx: f64, sly: f64) {
        let primitive = PrimitiveConfig::short_position((ex, ey), (tpx, tpy), (slx, sly));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Forecast
    #[wasm_bindgen]
    pub fn forecast(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::forecast((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Bars pattern
    #[wasm_bindgen(js_name = barsPattern)]
    pub fn bars_pattern(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::bars_pattern((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Price projection
    #[wasm_bindgen(js_name = priceProjection)]
    pub fn price_projection(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        let primitive = PrimitiveConfig::price_projection((x1, y1), (x2, y2), (x3, y3));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Projection
    #[wasm_bindgen]
    pub fn projection(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::projection((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Volume Tools (3 primitives)
    // =========================================================================

    /// Anchored VWAP
    #[wasm_bindgen(js_name = anchoredVwap)]
    pub fn anchored_vwap(&mut self, x: f64, y: f64) {
        let primitive = PrimitiveConfig::anchored_vwap((x, y));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Fixed volume profile
    #[wasm_bindgen(js_name = fixedVolumeProfile)]
    pub fn fixed_volume_profile(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::fixed_volume_profile((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Anchored volume profile
    #[wasm_bindgen(js_name = anchoredVolumeProfile)]
    pub fn anchored_volume_profile(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::anchored_volume_profile((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Measurement (3 primitives)
    // =========================================================================

    /// Price range
    #[wasm_bindgen(js_name = priceRange)]
    pub fn price_range(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::price_range((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Date range
    #[wasm_bindgen(js_name = dateRange)]
    pub fn date_range(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::date_range((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Price and date range
    #[wasm_bindgen(js_name = priceDateRange)]
    pub fn price_date_range(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let primitive = PrimitiveConfig::price_date_range((x1, y1), (x2, y2));
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Brushes (2 primitives)
    // =========================================================================

    /// Brush
    #[wasm_bindgen]
    pub fn brush(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::brush(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    /// Highlighter
    #[wasm_bindgen]
    pub fn highlighter(&mut self, points: Vec<f64>) {
        let pts: Vec<(f64, f64)> = points.chunks(2).map(|c| (c[0], c[1])).collect();
        let primitive = PrimitiveConfig::highlighter(pts);
        let chart = self.take_inner().primitive(primitive);
        self.put_inner(chart);
    }

    // =========================================================================
    // Rendering
    // =========================================================================

    /// Render chart to SVG string
    #[wasm_bindgen(js_name = renderSvg)]
    pub fn render_svg(&self) -> String {
        self.inner
            .as_ref()
            .map(|c| c.render_svg())
            .unwrap_or_default()
    }
}

// =============================================================================
// JsViewport - Chart viewport management
// =============================================================================

/// Chart viewport managing visible area and scrolling.
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

// =============================================================================
// JsTheme - Color schemes
// =============================================================================

/// Chart theme with predefined color schemes.
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

// =============================================================================
// JsChartConfig - Low-level configuration
// =============================================================================

/// Low-level chart configuration.
#[wasm_bindgen]
pub struct JsChartConfig {
    #[allow(dead_code)]
    inner: RustChartConfig,
}

#[wasm_bindgen]
impl JsChartConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: RustChartConfig::default(),
        }
    }
}

impl Default for JsChartConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Module Functions
// =============================================================================

/// Get library version.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
