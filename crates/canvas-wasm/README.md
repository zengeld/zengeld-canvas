# zengeld-canvas (JavaScript/WASM)

WebAssembly bindings for the high-performance zengeld-canvas chart rendering engine.

## Installation

```bash
npm install zengeld-canvas
```

## Quick Start

```javascript
import init, { JsBar, JsViewport, JsPriceScale, JsTheme } from 'zengeld-canvas';

async function main() {
  await init();

  // Create bars
  const bar = new JsBar(1703721600, 100.0, 105.0, 98.0, 103.0, 1000000);
  console.log(`Bullish: ${bar.isBullish()}`);

  // Create viewport
  const viewport = new JsViewport(800.0, 600.0);
  viewport.firstBar = 0.0;
  viewport.lastBar = 100.0;

  // Create price scale
  const priceScale = new JsPriceScale();
  priceScale.setRange(95.0, 110.0);

  // Use dark theme
  const theme = JsTheme.dark();
  console.log(`Background: ${theme.background}`);
}

main();
```

## Build from source

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler
```

## License

MIT OR Apache-2.0
