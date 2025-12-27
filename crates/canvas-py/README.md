# zengeld-canvas (Python)

Python bindings for the high-performance zengeld-canvas chart rendering engine.

## Installation

```bash
pip install zengeld-canvas
```

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
