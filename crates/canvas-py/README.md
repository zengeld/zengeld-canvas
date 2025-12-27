# zengeld-canvas

**High-performance SVG chart rendering engine for financial data visualization**

[![Crates.io](https://img.shields.io/crates/v/zengeld-canvas.svg)](https://crates.io/crates/zengeld-canvas)
[![PyPI](https://img.shields.io/pypi/v/zengeld-canvas.svg)](https://pypi.org/project/zengeld-canvas/)
[![npm](https://img.shields.io/npm/v/zengeld-canvas.svg)](https://www.npmjs.com/package/zengeld-canvas)

Python bindings for the high-performance zengeld-canvas chart rendering engine. Built in Rust with zero runtime dependencies.

## Installation

```bash
pip install zengeld-canvas
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
  <tr>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/22_primitives_gann.svg" width="400"/><br/><b>Gann Tools</b></td>
    <td align="center"><img src="https://raw.githubusercontent.com/zengeld/zengeld-canvas/main/crates/canvas/chart_output/25_primitives_patterns.svg" width="400"/><br/><b>Chart Patterns</b></td>
  </tr>
</table>

## Features

- **80+ Drawing Primitives** - Fibonacci, Gann, Pitchforks, Patterns, Elliott Waves, and more
- **12 Series Types** - Candlestick, Line, Area, Histogram, and more
- **Platform Agnostic** - `RenderContext` trait for any rendering backend
- **Zero Dependencies** - Only serde for serialization
- **High Performance** - Optimized for real-time chart rendering

## Quick Start

```python
from zengeld_canvas import Bar, Viewport, PriceScale, Theme

# Create bars
bar = Bar(time=1703721600, open=100.0, high=105.0, low=98.0, close=103.0)
print(f"Bullish: {bar.is_bullish()}")

# Create viewport
viewport = Viewport(width=800.0, height=600.0)
viewport.first_bar = 0.0
viewport.last_bar = 100.0

# Create price scale
price_scale = PriceScale()
price_scale.set_range(95.0, 110.0)

# Use dark theme
theme = Theme.dark()
print(f"Background: {theme.background}")
```

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
