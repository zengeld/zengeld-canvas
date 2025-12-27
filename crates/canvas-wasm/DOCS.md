# zengeld-canvas JavaScript/WASM API Documentation

High-performance SVG chart rendering engine for financial data visualization.

## Installation

```bash
npm install zengeld-canvas
```

## Quick Start

```javascript
import init, { Chart, JsBar } from 'zengeld-canvas';

async function main() {
  await init();

  // Create sample OHLCV data
  const bars = [];
  for (let i = 0; i < 100; i++) {
    bars.push(new JsBar(
      BigInt(1704067200 + i * 60),
      100.0 + i * 0.5, 102.0 + i * 0.5,
      98.0 + i * 0.5, 101.0 + i * 0.5,
      1000.0 * (i + 1)
    ));
  }

  // Create chart
  const chart = new Chart(800, 600);
  chart.setBars(bars);
  chart.candlesticks();
  chart.sma(20, "#2196F3");
  chart.rsi(14);

  // Render to SVG
  const svg = chart.renderSvg();
  document.getElementById('chart').innerHTML = svg;
}

main();
```

## API Reference

### JsBar

OHLCV bar representing one time period of price data.

```javascript
const bar = new JsBar(timestamp, open, high, low, close, volume);
```

**Parameters:**
- `timestamp` (BigInt): Unix timestamp in seconds
- `open` (number): Opening price
- `high` (number): Highest price
- `low` (number): Lowest price
- `close` (number): Closing price
- `volume` (number): Trading volume

**Methods:**
- `isBullish(): boolean`: Returns true if close > open

---

### Chart

High-level chart builder for creating financial charts.

```javascript
const chart = new Chart(width, height);
```

#### Configuration

```javascript
chart.setDpr(2.0);           // Set device pixel ratio
chart.setBars(bars);         // Set OHLCV data
```

#### Series Types (3 types)

```javascript
chart.candlesticks();        // Candlestick chart (default)
chart.line();                // Line chart
chart.area();                // Area chart
```

#### Theme & Styling

```javascript
chart.setColors("#00FF00", "#FF0000");  // Set up/down colors
chart.setBackground("#1a1a2e");         // Set background
chart.setGrid(false);                   // Hide grid
chart.darkTheme();                      // Dark theme preset
chart.lightTheme();                     // Light theme preset
```

---

## Indicators (45+ types)

### Moving Averages (10 types)

```javascript
chart.sma(20, "#2196F3");       // Simple Moving Average
chart.ema(12, "#E91E63");       // Exponential Moving Average
chart.wma(20, "#FF9800");       // Weighted Moving Average
chart.hma(14, "#9C27B0");       // Hull Moving Average
chart.dema(20, "#00BCD4");      // Double Exponential MA
chart.tema(20, "#4CAF50");      // Triple Exponential MA
chart.kama(10, "#FF5722");      // Kaufman Adaptive MA
chart.trima(20, "#3F51B5");     // Triangular MA
chart.zlema(20, "#795548");     // Zero Lag EMA
chart.mcginley(14, "#607D8B");  // McGinley Dynamic
```

### Band Indicators (5 types)

```javascript
chart.bollinger(20, 2.0);       // Bollinger Bands
chart.bollingerFilled(20);      // Bollinger Bands (filled)
chart.keltner(20);              // Keltner Channel
chart.donchian(20);             // Donchian Channel
chart.atrBands(14, 2.0);        // ATR Bands
```

### Oscillators (17 types)

```javascript
chart.rsi(14);                  // Relative Strength Index
chart.macd(12, 26, 9);          // MACD
chart.macdDefault();            // MACD (12, 26, 9)
chart.stochastic(14, 3);        // Stochastic Oscillator
chart.stochRsi(14);             // Stochastic RSI
chart.cci(20);                  // Commodity Channel Index
chart.williamsR(14);            // Williams %R
chart.momentum(10);             // Momentum
chart.roc(12);                  // Rate of Change
chart.tsi(25, 13);              // True Strength Index
chart.ultimateOscillator();     // Ultimate Oscillator
chart.awesomeOscillator();      // Awesome Oscillator
chart.acceleratorOscillator();  // Accelerator Oscillator
chart.cmo(14);                  // Chande Momentum Oscillator
chart.dpo(20);                  // Detrended Price Oscillator
chart.kst();                    // Know Sure Thing
chart.rvi(10);                  // Relative Vigor Index
```

### Volatility Indicators (6 types)

```javascript
chart.atr(14);                  // Average True Range
chart.stddev(20);               // Standard Deviation
chart.historicalVolatility(20); // Historical Volatility
chart.choppiness(14);           // Choppiness Index
chart.massIndex();              // Mass Index
chart.ulcerIndex(14);           // Ulcer Index
```

### Trend Indicators (12 types)

```javascript
chart.adx(14);                  // Average Directional Index
chart.aroon(25);                // Aroon
chart.aroonOscillator(25);      // Aroon Oscillator
chart.vortex(14);               // Vortex Indicator
chart.linearRegression(14, "#2196F3");  // Linear Regression
chart.linearRegressionSlope(14);        // Linear Regression Slope
chart.zigzag(5.0);              // ZigZag
chart.trix(15);                 // TRIX
chart.chandeKrollStop();        // Chande Kroll Stop
chart.psar();                   // Parabolic SAR
chart.supertrend(10, 3.0);      // Supertrend
chart.ichimoku();               // Ichimoku Cloud
```

### Volume Indicators (12 types)

```javascript
chart.volume();                 // Volume histogram
chart.obv();                    // On Balance Volume
chart.adLine();                 // Accumulation/Distribution
chart.cmf(20);                  // Chaikin Money Flow
chart.chaikinOscillator();      // Chaikin Oscillator
chart.vpt();                    // Volume Price Trend
chart.forceIndex(13);           // Force Index
chart.eom(14);                  // Ease of Movement
chart.nvi();                    // Negative Volume Index
chart.pvi();                    // Positive Volume Index
chart.mfi(14);                  // Money Flow Index
chart.vwap();                   // VWAP
```

### Specialized Indicators (12 types)

```javascript
chart.elderRay(13);             // Elder Ray
chart.balanceOfPower();         // Balance of Power
chart.connorsRsi();             // Connors RSI
chart.coppockCurve();           // Coppock Curve
chart.fisherTransform(10);      // Fisher Transform
chart.smiErgodic();             // SMI Ergodic
chart.schaffTrendCycle();       // Schaff Trend Cycle
chart.pgo(14);                  // Pretty Good Oscillator
chart.qstick(14);               // QStick
chart.pivotPoints();            // Pivot Points
chart.envelopes(20, 2.5);       // Envelopes
chart.priceChannel(20);         // Price Channel
```

### Custom Overlay

```javascript
chart.addOverlay("Custom", values, "#FF5722");
```

---

## Signals (7 types)

```javascript
chart.buySignal(25, 105.0);                // Buy signal
chart.buySignal(25, 105.0, "Long");        // With label
chart.sellSignal(45, 125.0);               // Sell signal
chart.entrySignal(30, 110.0);              // Entry signal
chart.exitSignal(50, 120.0);               // Exit signal
chart.takeProfitSignal(55, 130.0);         // Take profit
chart.stopLossSignal(35, 95.0);            // Stop loss
chart.customSignal(40, 115.0, "Alert");    // Custom signal
```

---

## Drawing Primitives (96 types)

### Lines (9 types)

```javascript
chart.horizontalLine(100.0);                      // Horizontal line
chart.verticalLine(50);                           // Vertical line
chart.trendLine(10, 100.0, 50, 120.0);            // Trend line
chart.ray(10, 100.0, 50, 120.0);                  // Ray
chart.extendedLine(10, 100.0, 50, 120.0);         // Extended line
chart.infoLine(10, 100.0, 50, 120.0);             // Info line
chart.trendAngle(10, 100.0, 50, 120.0);           // Trend angle
chart.horizontalRay(10, 100.0, 50, 100.0);        // Horizontal ray
chart.crossLine(50, 100.0);                       // Cross line
```

### Channels (4 types)

```javascript
chart.parallelChannel(0, 100, 50, 120, 0, 90);
chart.regressionTrend(10, 100, 50, 120);
chart.flatTopBottom(0, 100, 50, 100, 25, 90);
chart.disjointChannel([0, 100, 20, 110, 40, 105]);  // Flat array of x,y pairs
```

### Shapes (10 types)

```javascript
chart.rectangle(10, 100, 50, 120);
chart.circle(30, 110, 40, 120);
chart.ellipse(30, 110, 50, 130);
chart.triangle(10, 100, 50, 100, 30, 130);
chart.arc(10, 100, 30, 120, 50, 100);
chart.polyline([10, 100, 20, 110, 30, 105]);        // Flat array
chart.path([10, 100, 50, 100, 50, 120, 10, 120]);   // Flat array
chart.rotatedRectangle(10, 100, 50, 120, 20, 80);
chart.curve([10, 100, 30, 130, 50, 100]);           // Flat array
chart.doubleCurve([10, 100, 30, 130, 50, 100]);     // Flat array
```

### Fibonacci (11 types)

```javascript
chart.fibRetracement(10, 100, 50, 150);
chart.fibExtension(10, 100, 30, 130, 40, 115);
chart.fibChannel(10, 100, 50, 120, 10, 80);
chart.fibTimeZones(10, 100, 30, 100);
chart.fibSpeedResistance(10, 100, 50, 150);
chart.fibTrendTime(10, 100, 30, 130, 50, 115);
chart.fibCircles(10, 100, 50, 150);
chart.fibSpiral(10, 100, 50, 150);
chart.fibArcs(10, 100, 50, 150);
chart.fibWedge(10, 100, 50, 150);
chart.fibFan(10, 100, 50, 150);
```

### Pitchforks (4 types)

```javascript
chart.pitchfork(10, 100, 30, 130, 30, 90);
chart.schiffPitchfork(10, 100, 30, 130, 30, 90);
chart.modifiedSchiff(10, 100, 30, 130, 30, 90);
chart.insidePitchfork(10, 100, 30, 130, 30, 90);
```

### Gann (4 types)

```javascript
chart.gannBox(10, 100, 50, 150);
chart.gannSquareFixed(10, 100, 50, 150);
chart.gannSquare(10, 100, 50, 150);
chart.gannFan(10, 100, 50, 150);
```

### Patterns (6 types)

```javascript
chart.xabcdPattern([0, 100, 10, 120, 20, 110, 30, 140, 40, 115]);
chart.cypherPattern([0, 100, 10, 120, 20, 110, 30, 140, 40, 115]);
chart.headShoulders([0, 100, 10, 120, 20, 100, 30, 140, 40, 100, 50, 120, 60, 100]);
chart.abcdPattern([0, 100, 10, 120, 20, 110, 30, 130]);
chart.trianglePattern(10, 100, 50, 100, 30, 130);
chart.threeDrives([0, 100, 10, 120, 20, 105, 30, 130, 40, 110, 50, 140]);
```

### Elliott Waves (5 types)

```javascript
chart.elliottImpulse([0, 100, 10, 120, 15, 110, 25, 140, 30, 125, 40, 160]);
chart.elliottCorrection([0, 160, 10, 140, 15, 150, 25, 130]);
chart.elliottTriangle([0, 100, 10, 120, 20, 105, 30, 115, 40, 110]);
chart.elliottDoubleCombo([0, 100, 10, 120, 20, 105, 25, 115, 30, 100, 40, 130]);
chart.elliottTripleCombo([0, 100, 10, 120, ...]);
```

### Arrows (4 types)

```javascript
chart.arrowMarker(10, 100, 30, 120);
chart.arrowLine(10, 100, 30, 120);
chart.arrowUp(20, 95);
chart.arrowDown(20, 125);
```

### Annotations (11 types)

```javascript
chart.text(20, 100, "Label");
chart.anchoredText(10, 100, 50, 100, "Range");
chart.note(20, 100, "Note");
chart.priceNote(20, 100, "Price note");
chart.signpost(20, 100, "Signpost");
chart.callout(20, 100, "Callout");
chart.comment(20, 100, "Comment");
chart.priceLabel(20, 100);
chart.sign(20, 100);
chart.flag(20, 100);
chart.table(20, 100);
```

### Cycles (3 types)

```javascript
chart.cycleLines(10, 100, 30, 100);
chart.timeCycles(10, 100, 30, 100);
chart.sineWave(10, 100, 50, 100);
```

### Projections (6 types)

```javascript
chart.longPosition(20, 100, 20, 120, 20, 90);    // entry, tp, sl
chart.shortPosition(20, 120, 20, 100, 20, 130);  // entry, tp, sl
chart.forecast(10, 100, 50, 120);
chart.barsPattern(10, 100, 30, 120);
chart.priceProjection(10, 100, 30, 130, 50, 110);
chart.projection(10, 100, 50, 120);
```

### Volume Tools (3 types)

```javascript
chart.anchoredVwap(20, 100);
chart.fixedVolumeProfile(10, 100, 50, 150);
chart.anchoredVolumeProfile(10, 100, 50, 150);
```

### Measurement (3 types)

```javascript
chart.priceRange(10, 100, 50, 150);
chart.dateRange(10, 100, 50, 100);
chart.priceDateRange(10, 100, 50, 150);
```

### Brushes (2 types)

```javascript
chart.brush([10, 100, 20, 110, 30, 105]);        // Flat array
chart.highlighter([10, 100, 50, 100]);           // Flat array
```

---

## Rendering

```javascript
const svg = chart.renderSvg();

// Display in DOM
document.getElementById('chart').innerHTML = svg;

// Or create downloadable blob
const blob = new Blob([svg], { type: 'image/svg+xml' });
const url = URL.createObjectURL(blob);
```

---

## Helper Classes

### JsTheme

```javascript
import { JsTheme } from 'zengeld-canvas';

const dark = JsTheme.dark();
const light = JsTheme.light();

console.log(dark.bgColor);      // '#131722'
console.log(dark.candleUp);     // '#26a69a'
console.log(dark.candleDown);   // '#ef5350'
```

### JsViewport

```javascript
import { JsViewport } from 'zengeld-canvas';

const viewport = new JsViewport(800, 600);
viewport.setBarCount(100);
viewport.scrollToEnd();
```

### Utility

```javascript
import { version } from 'zengeld-canvas';
console.log(version());  // '0.1.7'
```

---

## React Example

```jsx
import React, { useEffect, useState } from 'react';
import init, { Chart, JsBar } from 'zengeld-canvas';

function ChartComponent({ data }) {
  const [svg, setSvg] = useState('');

  useEffect(() => {
    async function renderChart() {
      await init();
      const bars = data.map(d => new JsBar(
        BigInt(d.timestamp), d.open, d.high, d.low, d.close, d.volume
      ));

      const chart = new Chart(800, 600);
      chart.setBars(bars);
      chart.candlesticks();
      chart.darkTheme();
      chart.sma(20, "#2196F3");
      chart.rsi(14);

      setSvg(chart.renderSvg());
    }
    renderChart();
  }, [data]);

  return <div dangerouslySetInnerHTML={{ __html: svg }} />;
}
```

---

## Feature Summary

| Category | Count |
|----------|-------|
| Series Types | 3 |
| Moving Averages | 10 |
| Band Indicators | 5 |
| Oscillators | 17 |
| Volatility | 6 |
| Trend | 12 |
| Volume | 12 |
| Specialized | 12 |
| **Total Indicators** | **74** |
| Signal Types | 7 |
| Drawing Primitives | 96 |
| **Total API Methods** | **180+** |

## Browser Compatibility

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 79+

Requires WebAssembly support.

## License

MIT OR Apache-2.0
