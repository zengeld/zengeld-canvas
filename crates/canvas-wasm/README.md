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
- **Theme System** - 4 built-in presets (dark, light, high_contrast, cyberpunk) + runtime customization

## Installation

```bash
npm install zengeld-canvas
```

## Quick Start

```javascript
import init, { Chart, JsBar, JsUITheme, JsRuntimeTheme } from 'zengeld-canvas';

await init();

// Create chart
const chart = new Chart(800, 600);
chart.setBars(bars);
chart.candlesticks();
chart.sma(20, "#2196F3");
const svg = chart.renderSvg();

// With theme preset
const theme = JsUITheme.cyberpunk();
const chart2 = new Chart(800, 600);
chart2.setBars(bars);
chart2.candlesticks();
chart2.background(theme.background);
chart2.colors(theme.candle_up_body, theme.candle_down_body);

// Runtime theme (modifiable)
const runtime = JsRuntimeTheme.fromPreset("dark");
runtime.background = "#1a0a2e";  // Custom background
runtime.candle_up_body = "#00ffff";  // Cyan
const json = runtime.toJson();
```

## Examples

<table>
  <tr>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/09_dark_theme.svg" width="400"/><br/><b>Dark Theme</b></td>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/09_light_theme.svg" width="400"/><br/><b>Light Theme</b></td>
  </tr>
  <tr>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/09b_high_contrast_theme.svg" width="400"/><br/><b>High Contrast Theme</b></td>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/09c_cyberpunk_theme.svg" width="400"/><br/><b>Cyberpunk Theme</b></td>
  </tr>
</table>

## Theme System

Built-in presets: `dark()`, `light()`, `highContrast()`, `cyberpunk()`

```javascript
import { JsUITheme, JsRuntimeTheme } from 'zengeld-canvas';

// Static themes
const dark = JsUITheme.dark();
const light = JsUITheme.light();

// Runtime themes (modifiable, JSON support)
const runtime = JsRuntimeTheme.fromPreset("dark");
runtime.background = "#1a0a2e";
const json = runtime.toJson();

// Available presets
const presets = JsRuntimeTheme.presets();  // ["dark", "light", "high_contrast", "cyberpunk"]
```

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
