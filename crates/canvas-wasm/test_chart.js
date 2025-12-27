/**
 * zengeld-canvas WASM/JavaScript Test/Example
 *
 * Demonstrates the JavaScript API for chart rendering with various themes and primitives.
 * This script mirrors the Rust chart_gallery example to verify WASM bindings work correctly.
 *
 * Usage (Node.js):
 *   1. Build the WASM package: wasm-pack build --target nodejs
 *   2. Run this script: node test_chart.js
 *
 * Usage (Browser):
 *   1. Build the WASM package: wasm-pack build --target web
 *   2. Import and use in your web application
 *
 * Output:
 *   - chart_output_js/*.svg files
 */

const fs = require('fs');
const path = require('path');

// Import the WASM module (adjust path based on your build output)
const wasm = require('./pkg/zengeld_canvas_wasm.js');

/**
 * Simple pseudo-random number generator (deterministic for reproducibility)
 */
function pseudoRandom(seed) {
    let x = BigInt(seed) * 1103515245n + 12345n;
    x = x * 1103515245n + 12345n;
    return Number((x >> 16n) & 0x7fffn) / 32767.0;
}

/**
 * Generate sample OHLCV bars with realistic price movement
 */
function generateSampleBars(count) {
    const bars = [];
    let price = 100.0;
    const baseVolume = 1_000_000.0;
    const startTime = 1700000000;

    for (let i = 0; i < count; i++) {
        // Random walk with trend
        const trend = Math.sin((i / count) * Math.PI * 2.0) * 10.0;
        const noise = pseudoRandom(i) * 4.0 - 2.0;
        const change = trend * 0.1 + noise;

        price += change;
        price = Math.max(price, 50.0);

        const volatility = 1.0 + pseudoRandom(i + 1000) * 2.0;
        const high = price + volatility;
        const low = price - volatility;

        let open, close;
        if (pseudoRandom(i + 2000) > 0.5) {
            open = low + pseudoRandom(i + 3000) * (high - low);
        } else {
            open = high - pseudoRandom(i + 4000) * (high - low);
        }

        if (pseudoRandom(i + 5000) > 0.5) {
            close = low + pseudoRandom(i + 6000) * (high - low);
        } else {
            close = high - pseudoRandom(i + 7000) * (high - low);
        }

        const volume = baseVolume * (0.5 + pseudoRandom(i + 8000) * 1.5);

        bars.push(new wasm.JsBar(
            BigInt(startTime + (i * 86400)),
            open,
            high,
            low,
            close,
            volume
        ));
    }

    return bars;
}

/**
 * Save SVG string to file
 */
function saveSvg(svg, filePath) {
    fs.writeFileSync(filePath, svg, 'utf8');
    console.log(`  -> Saved: ${filePath}`);
}

/**
 * Main test function
 */
async function main() {
    console.log(`zengeld-canvas JavaScript/WASM Test (v${wasm.version()})\n`);

    // Create output directory
    const outputDir = 'chart_output_js';
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }

    // Generate sample data
    const bars = generateSampleBars(200);
    console.log(`Generated ${bars.length} sample bars\n`);

    // =========================================================================
    // Theme Showcase
    // =========================================================================

    // 01. Dark Theme (default)
    console.log("01. Dark Theme Chart");
    let chart = new wasm.Chart(800, 400);
    chart.setBars(bars);
    chart.candlesticks();
    chart.darkTheme();
    chart.sma(20, "#2196F3");
    chart.ema(50, "#FF9800");
    let svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '01_dark_theme.svg'));

    // 02. Light Theme
    console.log("02. Light Theme Chart");
    const light = wasm.JsUITheme.light();
    chart = new wasm.Chart(800, 400);
    chart.setBars(bars);
    chart.candlesticks();
    chart.setBackground(light.background);
    chart.setColors(light.candleUpBody, light.candleDownBody);
    chart.sma(20, "#2196F3");
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '02_light_theme.svg'));

    // 03. High Contrast Theme
    console.log("03. High Contrast Theme Chart");
    const contrast = wasm.JsUITheme.highContrast();
    chart = new wasm.Chart(800, 400);
    chart.setBars(bars);
    chart.candlesticks();
    chart.setBackground(contrast.background);
    chart.setColors(contrast.candleUpBody, contrast.candleDownBody);
    chart.sma(20, contrast.accent);
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '03_high_contrast.svg'));

    // 04. Cyberpunk Theme
    console.log("04. Cyberpunk Theme Chart");
    const cyber = wasm.JsUITheme.cyberpunk();
    chart = new wasm.Chart(800, 400);
    chart.setBars(bars);
    chart.candlesticks();
    chart.setBackground(cyber.background);
    chart.setColors(cyber.candleUpBody, cyber.candleDownBody);
    chart.sma(20, cyber.accent);
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '04_cyberpunk.svg'));

    // 05. Runtime Theme (custom modified)
    console.log("05. Runtime Theme (Custom)");
    const runtime = wasm.JsRuntimeTheme.fromPreset("dark");
    runtime.background = "#1a0a2e";  // Deep purple
    runtime.candleUpBody = "#00ffff";  // Cyan
    runtime.candleDownBody = "#ff00ff";  // Magenta
    chart = new wasm.Chart(800, 400);
    chart.setBars(bars);
    chart.candlesticks();
    chart.setBackground(runtime.background);
    chart.setColors(runtime.candleUpBody, runtime.candleDownBody);
    chart.sma(20, "#ffff00");
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '05_runtime_custom.svg'));

    // =========================================================================
    // Indicators
    // =========================================================================

    // 06. Chart with MACD
    console.log("06. Chart with MACD");
    chart = new wasm.Chart(800, 500);
    chart.setBars(bars);
    chart.candlesticks();
    chart.ema(12, "#2196F3");
    chart.ema(26, "#FF9800");
    chart.macd(12, 26, 9);
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '06_with_macd.svg'));

    // 07. Chart with RSI
    console.log("07. Chart with RSI");
    chart = new wasm.Chart(800, 500);
    chart.setBars(bars);
    chart.candlesticks();
    chart.sma(20, "#2196F3");
    chart.rsi(14);
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '07_with_rsi.svg'));

    // 08. Chart with Bollinger Bands
    console.log("08. Chart with Bollinger Bands");
    chart = new wasm.Chart(800, 400);
    chart.setBars(bars);
    chart.candlesticks();
    chart.bollinger(20, 2.0);
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '08_bollinger.svg'));

    // =========================================================================
    // Primitives
    // =========================================================================

    // 09. Lines and Shapes
    console.log("09. Lines and Shapes");
    chart = new wasm.Chart(1000, 500);
    chart.setBars(bars);
    chart.candlesticks();
    chart.trendLine([20.0, bars[20].low], [80.0, bars[80].low]);
    chart.horizontalLine(bars[50].high + 5.0);
    chart.verticalLine(100.0);
    chart.rectangle([30.0, bars[30].high], [60.0, bars[45].low]);
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '09_lines_shapes.svg'));

    // 10. Fibonacci
    console.log("10. Fibonacci Retracement");
    chart = new wasm.Chart(1000, 500);
    chart.setBars(bars);
    chart.candlesticks();
    chart.fibRetracement([20.0, bars[20].low], [80.0, bars[50].high]);
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '10_fibonacci.svg'));

    // =========================================================================
    // Signals
    // =========================================================================

    // 11. Trading Signals
    console.log("11. Trading Signals");
    chart = new wasm.Chart(1000, 500);
    chart.setBars(bars);
    chart.candlesticks();
    chart.sma(10, "#26a69a");
    chart.sma(30, "#ef5350");
    chart.buySignal(25, bars[25].low - 2.0, "Long");
    chart.sellSignal(60, bars[60].high + 2.0, "Short");
    chart.takeProfitSignal(45, bars[45].high + 1.0, "TP1");
    chart.stopLossSignal(95, bars[95].high + 3.0, "SL");
    svg = chart.renderSvg();
    saveSvg(svg, path.join(outputDir, '11_signals.svg'));

    // =========================================================================
    // API Verification
    // =========================================================================

    console.log("\n--- API Verification ---");

    // Test Viewport
    const vp = new wasm.JsViewport(1200.0, 800.0);
    console.log(`Viewport: ${vp.chartWidth}x${vp.chartHeight}, barWidth=${vp.barWidth.toFixed(2)}`);

    // Test JsUITheme presets
    console.log(`\nJsUITheme presets:`);
    const themes = [
        { name: 'dark', fn: () => wasm.JsUITheme.dark() },
        { name: 'light', fn: () => wasm.JsUITheme.light() },
        { name: 'highContrast', fn: () => wasm.JsUITheme.highContrast() },
        { name: 'cyberpunk', fn: () => wasm.JsUITheme.cyberpunk() },
    ];
    for (const { name, fn } of themes) {
        const theme = fn();
        console.log(`  - ${theme.name}: bg=${theme.background}, up=${theme.candleUpBody}`);
    }

    // Test JsRuntimeTheme
    console.log(`\nJsRuntimeTheme presets: ${wasm.JsRuntimeTheme.presets()}`);

    // Test JSON serialization
    const rt = wasm.JsRuntimeTheme.dark();
    const jsonStr = rt.toJson();
    console.log(`\nJsRuntimeTheme JSON length: ${jsonStr.length} chars`);

    // Test JSON roundtrip
    const rt2 = wasm.JsRuntimeTheme.fromJson(jsonStr);
    if (rt2) {
        console.log(`JSON roundtrip: OK (name=${rt2.name})`);
    } else {
        console.log("JSON roundtrip: FAILED");
    }

    // Test JsBar
    const bar = new wasm.JsBar(1700000000n, 100.0, 105.0, 98.0, 103.0, 1000000.0);
    console.log(`\nJsBar: ts=${bar.timestamp}, o=${bar.open}, h=${bar.high}, l=${bar.low}, c=${bar.close}`);
    console.log(`  isBullish: ${bar.isBullish()}`);

    console.log(`\n[OK] All charts generated in '${outputDir}/'\n`);
    console.log("Generated files:");
    for (let i = 1; i <= 11; i++) {
        const prefix = i < 10 ? "0" : "";
        console.log(`  - ${prefix}${i}_*.svg`);
    }
}

// Run the test
main().catch(console.error);
