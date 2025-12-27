# zengeld-canvas

**High-performance SVG chart rendering engine for financial data visualization**

[![npm](https://img.shields.io/npm/v/zengeld-canvas.svg)](https://www.npmjs.com/package/zengeld-canvas)

WebAssembly bindings for the zengeld-canvas chart rendering engine. Built in Rust with zero runtime dependencies.

## Features

- **96 Drawing Primitives** - Fibonacci, Gann, Pitchforks, Elliott Waves, Patterns, Channels, and more
- **45+ Indicator Presets** - Pre-configured rendering styles for SMA, RSI, MACD, Bollinger, Ichimoku, etc.
- **12 Series Types** - Candlestick, HeikinAshi, Line, Area, Histogram, Baseline, and more
- **14 Multi-Chart Layouts** - Grid, split, and custom layouts for dashboards
- **High Performance** - Native Rust speed via WebAssembly

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
  const bars = [
    new JsBar(1703721600n, 100.0, 105.0, 98.0, 103.0, 1000.0)
  ];

  // Create chart
  const chart = new Chart(800, 600);
  chart.setBars(bars);
  chart.candlesticks();
  chart.sma(20, "#2196F3");

  // Render to SVG
  const svg = chart.renderSvg();

  // Use in DOM
  document.getElementById('chart').innerHTML = svg;
}

main();
```

## Examples

<table>
  <tr>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/09_dark_theme.svg" width="400"/><br/><b>Dark Theme</b></td>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/05_with_macd.svg" width="400"/><br/><b>MACD Indicator</b></td>
  </tr>
  <tr>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/14_multichart_1_3.svg" width="400"/><br/><b>Multi-Chart Layout</b></td>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/19_primitives_channels.svg" width="400"/><br/><b>Channels</b></td>
  </tr>
</table>

## Drawing Primitives

| Category | Count | Examples |
|----------|-------|----------|
| Fibonacci | 11 | Retracement, Fan, Arcs, Circles, Channel, Spiral |
| Lines | 9 | TrendLine, HorizontalLine, Ray, ExtendedLine |
| Annotations | 11 | Text, Callout, PriceLabel, Flag, Table |
| Shapes | 10 | Rectangle, Circle, Ellipse, Triangle, Path |
| Elliott Waves | 5 | Impulse, Correction, Triangle, Combo |
| Patterns | 6 | XABCD, HeadShoulders, Cypher, ThreeDrives |
| Gann | 4 | Fan, Box, Square, SquareFixed |
| And more... | 40 | Channels, Pitchforks, Cycles, Projections |

## License

MIT OR Apache-2.0

## Support the Project

If you find this library useful, consider supporting development:

| Currency | Network | Address |
|----------|---------|---------|
| USDT | TRC20 | `TNxMKsvVLYViQ5X5sgCYmkzH4qjhhh5U7X` |
| USDC | Arbitrum | `0xEF3B94Fe845E21371b4C4C5F2032E1f23A13Aa6e` |
| ETH | Ethereum | `0xEF3B94Fe845E21371b4C4C5F2032E1f23A13Aa6e` |
| BTC | Bitcoin | `bc1qjgzthxja8umt5tvrp5tfcf9zeepmhn0f6mnt40` |
| SOL | Solana | `DZJjmH8Cs5wEafz5Ua86wBBkurSA4xdWXa3LWnBUR94c` |

---

<p align="center">
  <a href="https://zen-geldmaschine.net/">
    <img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/assets/author.svg" alt="zengeld" />
  </a>
</p>
