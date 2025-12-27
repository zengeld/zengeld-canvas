//! Chart Gallery Example
//!
//! Demonstrates various chart configurations with different:
//! - Series types (candlesticks, line, area)
//! - Indicators (SMA, EMA, Bollinger, RSI, MACD)
//! - Subpanes
//! - Signals
//! - Primitives

use std::fs;
use zengeld_canvas::api::{
    Chart, ChartConfig, ChartRenderer, Indicator, MultichartRenderer, PrimitiveConfig,
    SeriesConfig, SignalConfig, ThemeConfig,
};
use zengeld_canvas::core::Bar;
use zengeld_canvas::layout::MultichartLayout;

fn main() {
    // Create output directory
    let output_dir = "chart_output";
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Generate sample data
    let bars = generate_sample_bars(200);

    println!("Generating chart gallery...\n");

    // 1. Simple Candlestick Chart
    println!("1. Simple Candlestick Chart");
    let svg = Chart::new(800, 400).bars(&bars).candlesticks().render_svg();
    save_svg(&svg, &format!("{}/01_candlestick.svg", output_dir));

    // 2. Line Chart with SMA
    println!("2. Line Chart with SMA");
    let svg = Chart::new(800, 400)
        .bars(&bars)
        .line()
        .sma(20, "#2196F3")
        .sma(50, "#FF9800")
        .render_svg();
    save_svg(&svg, &format!("{}/02_line_with_sma.svg", output_dir));

    // 3. Candlestick with Bollinger Bands
    println!("3. Candlestick with Bollinger Bands");
    let svg = Chart::new(800, 400)
        .bars(&bars)
        .candlesticks()
        .bollinger(20, 2.0)
        .render_svg();
    save_svg(&svg, &format!("{}/03_bollinger.svg", output_dir));

    // 4. Chart with RSI Subpane
    println!("4. Chart with RSI Subpane");
    let svg = Chart::new(800, 500)
        .bars(&bars)
        .candlesticks()
        .sma(20, "#2196F3")
        .rsi(14)
        .render_svg();
    save_svg(&svg, &format!("{}/04_with_rsi.svg", output_dir));

    // 5. Chart with MACD Subpane
    println!("5. Chart with MACD Subpane");
    let svg = Chart::new(800, 500)
        .bars(&bars)
        .candlesticks()
        .ema(12, "#2196F3")
        .ema(26, "#FF9800")
        .macd(12, 26, 9)
        .render_svg();
    save_svg(&svg, &format!("{}/05_with_macd.svg", output_dir));

    // 6. Full Featured Chart (RSI + Volume)
    println!("6. Full Featured Chart");
    let svg = Chart::new(1000, 700)
        .bars(&bars)
        .candlesticks()
        .sma(20, "#2196F3")
        .sma(50, "#FF9800")
        .ema(9, "#26a69a")
        .rsi(14)
        .volume()
        .render_svg();
    save_svg(&svg, &format!("{}/06_full_featured.svg", output_dir));

    // 7. Chart with Buy/Sell Signals
    println!("7. Chart with Signals");
    let svg = Chart::new(800, 400)
        .bars(&bars)
        .candlesticks()
        .sma(20, "#2196F3")
        .signal(SignalConfig::buy(30, bars[30].low - 2.0))
        .signal(SignalConfig::sell(60, bars[60].high + 2.0))
        .signal(SignalConfig::buy(90, bars[90].low - 2.0))
        .signal(SignalConfig::sell(120, bars[120].high + 2.0))
        .signal(SignalConfig::buy(150, bars[150].low - 2.0))
        .render_svg();
    save_svg(&svg, &format!("{}/07_with_signals.svg", output_dir));

    // 8. Chart with Primitives (Trend Lines)
    println!("8. Chart with Primitives");
    let svg = Chart::new(800, 400)
        .bars(&bars)
        .candlesticks()
        .primitive(PrimitiveConfig::trend_line(
            (20.0, bars[20].low as f64),
            (80.0, bars[80].low as f64),
        ))
        .primitive(PrimitiveConfig::horizontal_line(
            bars.iter()
                .map(|b| b.high)
                .fold(f64::NEG_INFINITY, f64::max)
                + 1.0,
        ))
        .render_svg();
    save_svg(&svg, &format!("{}/08_with_primitives.svg", output_dir));

    // 9. Dark Theme Chart
    println!("9. Dark Theme Chart");
    let svg = Chart::new(800, 400)
        .bars(&bars)
        .candlesticks()
        .background("#000000")
        .colors("#00ff00", "#ff0000")
        .sma(20, "#ffff00")
        .render_svg();
    save_svg(&svg, &format!("{}/09_dark_theme.svg", output_dir));

    // 10. Area Chart
    println!("10. Area Chart");
    let svg = Chart::new(800, 400).bars(&bars).area().render_svg();
    save_svg(&svg, &format!("{}/10_area.svg", output_dir));

    // 11. Using ChartConfig directly (advanced)
    println!("11. ChartConfig Direct Usage");
    let config = ChartConfig {
        width: 1200,
        height: 800,
        dpr: 2.0,
        theme: ThemeConfig {
            background: "#1a1a2e".into(),
            up_color: "#00d4aa".into(),
            down_color: "#ff6b6b".into(),
            grid_color: "#2a2a4a".into(),
            text_color: "#e0e0e0".into(),
            show_grid: true,
            ..Default::default()
        },
        series: SeriesConfig::candlestick(),
        indicators: vec![
            // Overlays (on main chart)
            Indicator::sma("sma_10", 10, "#4ecdc4"),
            Indicator::sma("sma_20", 20, "#45b7d1"),
            Indicator::ema("ema_50", 50, "#96ceb4"),
            // Subpanes (separate panels)
            Indicator::rsi("rsi_14", 14),
        ],
        primitives: vec![],
        signals: vec![
            SignalConfig::buy(40, bars[40].low - 3.0),
            SignalConfig::sell(100, bars[100].high + 3.0),
        ],
        ..Default::default()
    };

    let renderer = ChartRenderer::new(&config, &bars);
    let svg = renderer.render_svg();
    save_svg(&svg, &format!("{}/11_config_direct.svg", output_dir));

    // 12. Multiple Overlays Comparison
    println!("12. Multiple Overlays");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .sma(5, "#e91e63")
        .sma(10, "#9c27b0")
        .sma(20, "#673ab7")
        .sma(50, "#3f51b5")
        .sma(100, "#2196f3")
        .ema(21, "#00bcd4")
        .render_svg();
    save_svg(&svg, &format!("{}/12_multiple_overlays.svg", output_dir));

    // =========================================================================
    // Multichart Examples
    // =========================================================================

    // 13. 2x2 Grid - Different Series Types
    println!("13. Multichart 2x2 - Different Series Types");
    let layout_2x2 = MultichartLayout::quad();

    // Create 4 different chart configs
    let config_candlestick = ChartConfig {
        series: SeriesConfig::candlestick(),
        indicators: vec![Indicator::sma("sma", 20, "#2196F3")],
        ..Default::default()
    };
    let config_line = ChartConfig {
        series: SeriesConfig::line(),
        indicators: vec![Indicator::ema("ema", 12, "#FF9800")],
        ..Default::default()
    };
    let config_area = ChartConfig {
        series: SeriesConfig::area(),
        ..Default::default()
    };
    let config_bar = ChartConfig {
        series: SeriesConfig::bar(),
        indicators: vec![Indicator::bollinger("bb", 20)],
        ..Default::default()
    };

    let svg = MultichartRenderer::new(&layout_2x2, 1200, 800)
        .chart(&config_candlestick, &bars)
        .chart(&config_line, &bars)
        .chart(&config_area, &bars)
        .chart(&config_bar, &bars)
        .render_svg();
    save_svg(&svg, &format!("{}/13_multichart_2x2.svg", output_dir));

    // 14. 1+3 Layout - Main chart + indicators
    println!("14. Multichart 1+3 - Main + Indicators");
    let layout_1_3 = MultichartLayout::one_plus_three();

    let config_main = ChartConfig {
        series: SeriesConfig::candlestick(),
        indicators: vec![
            Indicator::sma("sma_20", 20, "#2196F3"),
            Indicator::sma("sma_50", 50, "#FF9800"),
            Indicator::bollinger("bb", 20),
        ],
        ..Default::default()
    };
    let config_rsi = ChartConfig {
        series: SeriesConfig::line(),
        indicators: vec![Indicator::rsi("rsi", 14)],
        ..Default::default()
    };
    let config_macd = ChartConfig {
        series: SeriesConfig::line(),
        indicators: vec![Indicator::macd("macd", 12, 26, 9)],
        ..Default::default()
    };
    let config_volume = ChartConfig {
        series: SeriesConfig::line(),
        indicators: vec![Indicator::volume("vol")],
        ..Default::default()
    };

    let svg = MultichartRenderer::new(&layout_1_3, 1200, 800)
        .chart(&config_main, &bars)
        .chart(&config_rsi, &bars)
        .chart(&config_macd, &bars)
        .chart(&config_volume, &bars)
        .render_svg();
    save_svg(&svg, &format!("{}/14_multichart_1_3.svg", output_dir));

    // 15. Horizontal Split - Two timeframes comparison
    println!("15. Multichart Horizontal - Two Charts");
    let layout_h2 = MultichartLayout::horizontal_split();

    let config_with_subpane1 = ChartConfig {
        series: SeriesConfig::candlestick(),
        indicators: vec![
            Indicator::sma("sma", 20, "#2196F3"),
            Indicator::rsi("rsi", 14),
        ],
        theme: ThemeConfig {
            up_color: "#26a69a".into(),
            down_color: "#ef5350".into(),
            ..Default::default()
        },
        ..Default::default()
    };
    let config_with_subpane2 = ChartConfig {
        series: SeriesConfig::candlestick(),
        indicators: vec![
            Indicator::ema("ema", 12, "#FF9800"),
            Indicator::macd("macd", 12, 26, 9),
        ],
        theme: ThemeConfig {
            up_color: "#4caf50".into(),
            down_color: "#f44336".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    let svg = MultichartRenderer::new(&layout_h2, 1400, 600)
        .chart(&config_with_subpane1, &bars)
        .chart(&config_with_subpane2, &bars)
        .render_svg();
    save_svg(
        &svg,
        &format!("{}/15_multichart_horizontal.svg", output_dir),
    );

    // 16. Vertical Stack - 3 charts with different series
    println!("16. Multichart Vertical 3 - Stack");
    let layout_v3 = MultichartLayout::triple_vertical();

    let config_heikin = ChartConfig {
        series: SeriesConfig::heikin_ashi(),
        indicators: vec![Indicator::sma("sma", 10, "#2196F3")],
        ..Default::default()
    };
    let config_baseline = ChartConfig {
        series: SeriesConfig::baseline(100.0), // baseline at 100
        ..Default::default()
    };
    let config_hollow = ChartConfig {
        series: SeriesConfig::hollow_candlestick(),
        indicators: vec![Indicator::ema("ema", 21, "#9c27b0")],
        ..Default::default()
    };

    let svg = MultichartRenderer::new(&layout_v3, 1000, 900)
        .chart(&config_heikin, &bars)
        .chart(&config_baseline, &bars)
        .chart(&config_hollow, &bars)
        .render_svg();
    save_svg(&svg, &format!("{}/16_multichart_vertical.svg", output_dir));

    // 17. Six Pack - 2x3 Grid
    println!("17. Multichart 2x3 - Six Pack");
    let layout_6 = MultichartLayout::six_pack();

    let configs = vec![
        ChartConfig {
            series: SeriesConfig::candlestick(),
            ..Default::default()
        },
        ChartConfig {
            series: SeriesConfig::hollow_candlestick(),
            ..Default::default()
        },
        ChartConfig {
            series: SeriesConfig::line(),
            ..Default::default()
        },
        ChartConfig {
            series: SeriesConfig::area(),
            ..Default::default()
        },
        ChartConfig {
            series: SeriesConfig::bar(),
            ..Default::default()
        },
        ChartConfig {
            series: SeriesConfig::baseline(100.0),
            ..Default::default()
        },
    ];

    let mut renderer = MultichartRenderer::new(&layout_6, 1500, 800);
    for config in &configs {
        renderer = renderer.chart(config, &bars);
    }
    let svg = renderer.render_svg();
    save_svg(&svg, &format!("{}/17_multichart_6pack.svg", output_dir));

    // =========================================================================
    // Primitives Showcase
    // =========================================================================

    // 18. Lines & Rays
    println!("18. Primitives - Lines & Rays");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(PrimitiveConfig::trend_line(
            (20.0, bars[20].low),
            (80.0, bars[80].low),
        ))
        .primitive(PrimitiveConfig::horizontal_line(bars[50].high + 5.0).with_color("#FF9800"))
        .primitive(PrimitiveConfig::vertical_line(100.0).with_color("#9C27B0"))
        .primitive(
            PrimitiveConfig::ray((30.0, bars[30].high), (70.0, bars[70].high))
                .with_color("#2196F3"),
        )
        .primitive(
            PrimitiveConfig::extended_line((10.0, bars[10].close), (50.0, bars[50].close))
                .with_color("#4CAF50"),
        )
        .primitive(
            PrimitiveConfig::horizontal_ray((60.0, bars[60].close), (90.0, bars[60].close))
                .with_color("#E91E63"),
        )
        .primitive(PrimitiveConfig::cross_line((120.0, bars[120].close)).with_color("#00BCD4"))
        .render_svg();
    save_svg(&svg, &format!("{}/18_primitives_lines.svg", output_dir));

    // 19. Channels
    println!("19. Primitives - Channels");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(
            PrimitiveConfig::parallel_channel(
                (20.0, bars[20].low),
                (80.0, bars[80].low),
                (20.0, bars[20].high + 3.0),
            )
            .with_color("#2196F3")
            .with_fill("#2196F3", 0.1),
        )
        .primitive(
            PrimitiveConfig::regression_trend((100.0, bars[100].close), (160.0, bars[160].close))
                .with_color("#FF9800"),
        )
        .render_svg();
    save_svg(&svg, &format!("{}/19_primitives_channels.svg", output_dir));

    // 20. Shapes
    println!("20. Primitives - Shapes");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(
            PrimitiveConfig::rectangle((30.0, bars[30].high), (60.0, bars[45].low))
                .with_color("#4CAF50")
                .with_fill("#4CAF50", 0.2),
        )
        .primitive(
            PrimitiveConfig::ellipse((100.0, bars[100].close), (115.0, bars[100].close + 5.0))
                .with_color("#E91E63")
                .with_fill("#E91E63", 0.15),
        )
        .primitive(
            PrimitiveConfig::triangle(
                (140.0, bars[140].low),
                (160.0, bars[160].high),
                (180.0, bars[180].low),
            )
            .with_color("#9C27B0")
            .with_fill("#9C27B0", 0.1),
        )
        .render_svg();
    save_svg(&svg, &format!("{}/20_primitives_shapes.svg", output_dir));

    // 21. Fibonacci Tools
    println!("21. Primitives - Fibonacci");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(PrimitiveConfig::fib_retracement(
            (20.0, bars[20].low),
            (80.0, bars[50].high),
        ))
        .primitive(
            PrimitiveConfig::fib_fan((100.0, bars[100].low), (140.0, bars[120].high))
                .with_color("#FF9800"),
        )
        .render_svg();
    save_svg(&svg, &format!("{}/21_primitives_fibonacci.svg", output_dir));

    // 22. Gann Tools
    println!("22. Primitives - Gann");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(PrimitiveConfig::gann_box(
            (30.0, bars[30].low),
            (90.0, bars[60].high),
        ))
        .primitive(
            PrimitiveConfig::gann_fan((120.0, bars[120].low), (180.0, bars[150].high))
                .with_color("#9C27B0"),
        )
        .render_svg();
    save_svg(&svg, &format!("{}/22_primitives_gann.svg", output_dir));

    // 23. Pitchforks
    println!("23. Primitives - Pitchforks");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(
            PrimitiveConfig::pitchfork(
                (20.0, bars[20].high), // pivot
                (50.0, bars[50].low),  // left
                (80.0, bars[80].high), // right
            )
            .with_color("#2196F3"),
        )
        .primitive(
            PrimitiveConfig::schiff_pitchfork(
                (100.0, bars[100].high),
                (130.0, bars[130].low),
                (160.0, bars[160].high),
            )
            .with_color("#FF9800"),
        )
        .render_svg();
    save_svg(
        &svg,
        &format!("{}/23_primitives_pitchforks.svg", output_dir),
    );

    // 24. Annotations
    println!("24. Primitives - Annotations");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(PrimitiveConfig::text(
            (50.0, bars[50].high + 3.0),
            "Important Level",
        ))
        .primitive(PrimitiveConfig::note(
            (80.0, bars[80].low - 2.0),
            "Support Zone",
        ))
        .primitive(PrimitiveConfig::callout(
            (120.0, bars[120].high + 2.0),
            "Breakout!",
        ))
        .primitive(PrimitiveConfig::flag((150.0, bars[150].high)))
        .primitive(PrimitiveConfig::arrow_up((30.0, bars[30].low - 1.0)).with_color("#4CAF50"))
        .primitive(PrimitiveConfig::arrow_down((60.0, bars[60].high + 1.0)).with_color("#F44336"))
        .render_svg();
    save_svg(
        &svg,
        &format!("{}/24_primitives_annotations.svg", output_dir),
    );

    // 25. Patterns
    println!("25. Primitives - Patterns");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(
            PrimitiveConfig::triangle_pattern(
                (40.0, bars[40].low),
                (70.0, bars[55].high),
                (100.0, bars[100].low),
            )
            .with_color("#2196F3"),
        )
        .primitive(
            PrimitiveConfig::abcd_pattern(vec![
                (120.0, bars[120].low),
                (140.0, bars[130].high),
                (155.0, bars[145].low),
                (175.0, bars[160].high),
            ])
            .with_color("#FF9800"),
        )
        .render_svg();
    save_svg(&svg, &format!("{}/25_primitives_patterns.svg", output_dir));

    // 26. Projections & Positions
    println!("26. Primitives - Projections & Positions");
    let entry_price = bars[80].close;
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(PrimitiveConfig::long_position(
            (80.0, entry_price),         // entry
            (100.0, entry_price + 10.0), // take profit
            (70.0, entry_price - 5.0),   // stop loss
        ))
        .primitive(PrimitiveConfig::short_position(
            (140.0, bars[140].close),
            (160.0, bars[140].close - 8.0),
            (130.0, bars[140].close + 4.0),
        ))
        .primitive(
            PrimitiveConfig::price_range((20.0, bars[20].low), (50.0, bars[35].high))
                .with_color("#9C27B0"),
        )
        .render_svg();
    save_svg(&svg, &format!("{}/26_primitives_positions.svg", output_dir));

    // =========================================================================
    // Signals Showcase
    // =========================================================================

    // 27. All Signal Types
    println!("27. Signals - All Types");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .sma(20, "#2196F3")
        .signal(SignalConfig::buy(20, bars[20].low - 2.0))
        .signal(SignalConfig::sell(40, bars[40].high + 2.0))
        .signal(SignalConfig::entry(60, bars[60].close).with_color("#00BCD4"))
        .signal(SignalConfig::exit(80, bars[80].close).with_color("#FF9800"))
        .signal(SignalConfig::take_profit(100, bars[100].high + 1.0))
        .signal(SignalConfig::stop_loss(120, bars[120].low - 1.0))
        .signal(SignalConfig::custom(140, bars[140].close, "S").with_color("#9C27B0"))
        .signal(SignalConfig::custom(160, bars[160].high + 1.0, "!").with_color("#E91E63"))
        .render_svg();
    save_svg(&svg, &format!("{}/27_signals_all_types.svg", output_dir));

    // 28. Trading Strategy Signals
    println!("28. Signals - Trading Strategy");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .sma(10, "#26a69a")
        .sma(30, "#ef5350")
        // SMA crossover signals (simulated)
        .signal(SignalConfig::buy(25, bars[25].low - 2.0).with_label("Long"))
        .signal(SignalConfig::take_profit(45, bars[45].high + 1.0).with_label("TP1"))
        .signal(SignalConfig::take_profit(55, bars[55].high + 1.0).with_label("TP2"))
        .signal(SignalConfig::exit(65, bars[65].close).with_label("Exit"))
        .signal(SignalConfig::sell(90, bars[90].high + 2.0).with_label("Short"))
        .signal(SignalConfig::stop_loss(95, bars[95].high + 3.0).with_label("SL"))
        .signal(SignalConfig::take_profit(110, bars[110].low - 1.0).with_label("TP"))
        .signal(SignalConfig::buy(140, bars[140].low - 2.0).with_label("Long"))
        .signal(SignalConfig::exit(170, bars[170].close).with_label("Close"))
        .render_svg();
    save_svg(&svg, &format!("{}/28_signals_strategy.svg", output_dir));

    // =========================================================================
    // Events Showcase
    // =========================================================================

    // 29. Technical Events
    println!("29. Events - Technical");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .sma(20, "#2196F3")
        .ema(50, "#FF9800")
        .primitive(PrimitiveConfig::crossover((35.0, bars[35].close)).with_color("#4CAF50"))
        .primitive(PrimitiveConfig::breakdown((85.0, bars[85].close)).with_color("#F44336"))
        .primitive(
            PrimitiveConfig::divergence((100.0, bars[100].low), (130.0, bars[130].low))
                .with_color("#9C27B0"),
        )
        .primitive(PrimitiveConfig::trend_event((150.0, bars[150].high)).with_color("#00BCD4"))
        .primitive(PrimitiveConfig::momentum_event((170.0, bars[170].close)).with_color("#E91E63"))
        .render_svg();
    save_svg(&svg, &format!("{}/29_events_technical.svg", output_dir));

    // 30. Zone Events
    println!("30. Events - Zones");
    let svg = Chart::new(1000, 500)
        .bars(&bars)
        .candlesticks()
        .primitive(
            PrimitiveConfig::zone_event((20.0, bars[30].high + 2.0), (60.0, bars[30].high + 5.0))
                .with_color("#4CAF50")
                .with_text("Resistance Zone"),
        )
        .primitive(
            PrimitiveConfig::zone_event((80.0, bars[100].low - 3.0), (140.0, bars[100].low))
                .with_color("#2196F3")
                .with_text("Support Zone"),
        )
        .primitive(PrimitiveConfig::volume_event((50.0, bars[50].low - 2.0)).with_color("#FF9800"))
        .primitive(PrimitiveConfig::custom_event(
            (120.0, bars[120].high + 2.0),
            "News",
        ))
        .render_svg();
    save_svg(&svg, &format!("{}/30_events_zones.svg", output_dir));

    // =========================================================================
    // Complex Combined Examples
    // =========================================================================

    // 31. Full Analysis Chart
    println!("31. Full Analysis Chart");
    let svg = Chart::new(1200, 700)
        .bars(&bars)
        .candlesticks()
        .sma(20, "#2196F3")
        .sma(50, "#FF9800")
        .bollinger(20, 2.0)
        .rsi(14)
        .volume()
        // Fibonacci retracement
        .primitive(PrimitiveConfig::fib_retracement(
            (30.0, bars[30].low),
            (70.0, bars[50].high),
        ))
        // Support/Resistance
        .primitive(PrimitiveConfig::horizontal_line(bars[50].high).with_color("#ef5350"))
        .primitive(PrimitiveConfig::horizontal_line(bars[30].low).with_color("#26a69a"))
        // Signals
        .signal(SignalConfig::buy(35, bars[35].low - 2.0))
        .signal(SignalConfig::sell(65, bars[65].high + 2.0))
        .signal(SignalConfig::buy(100, bars[100].low - 2.0))
        // Annotations
        .primitive(PrimitiveConfig::text(
            (50.0, bars[50].high + 6.0),
            "Swing High",
        ))
        .primitive(PrimitiveConfig::text(
            (30.0, bars[30].low - 4.0),
            "Swing Low",
        ))
        .render_svg();
    save_svg(&svg, &format!("{}/31_full_analysis.svg", output_dir));

    println!("\n[OK] All charts generated in '{}/'\n", output_dir);
    println!("Generated files:");
    for i in 1..=31 {
        let prefix = if i < 10 { "0" } else { "" };
        println!("  - {}{}_*.svg", prefix, i);
    }
}

/// Generate sample OHLCV bars with realistic price movement
fn generate_sample_bars(count: usize) -> Vec<Bar> {
    let mut bars = Vec::with_capacity(count);
    let mut price = 100.0;
    let base_volume = 1_000_000.0;
    let start_time = 1700000000i64; // Some timestamp

    for i in 0..count {
        // Random walk with trend
        let trend = ((i as f64 / count as f64) * std::f64::consts::PI * 2.0).sin() * 10.0;
        let noise = pseudo_random(i as u64) * 4.0 - 2.0;
        let change = trend * 0.1 + noise;

        price += change;
        price = price.max(50.0); // Floor price

        let volatility = 1.0 + pseudo_random(i as u64 + 1000) * 2.0;
        let high = price + volatility;
        let low = price - volatility;

        let open = if pseudo_random(i as u64 + 2000) > 0.5 {
            low + pseudo_random(i as u64 + 3000) * (high - low)
        } else {
            high - pseudo_random(i as u64 + 4000) * (high - low)
        };

        let close = if pseudo_random(i as u64 + 5000) > 0.5 {
            low + pseudo_random(i as u64 + 6000) * (high - low)
        } else {
            high - pseudo_random(i as u64 + 7000) * (high - low)
        };

        let volume = base_volume * (0.5 + pseudo_random(i as u64 + 8000) * 1.5);

        bars.push(Bar {
            timestamp: start_time + (i as i64 * 86400), // Daily bars
            open,
            high,
            low,
            close,
            volume,
        });
    }

    bars
}

/// Simple pseudo-random number generator (deterministic for reproducibility)
fn pseudo_random(seed: u64) -> f64 {
    let x = seed.wrapping_mul(1103515245).wrapping_add(12345);
    let x = x.wrapping_mul(1103515245).wrapping_add(12345);
    ((x >> 16) & 0x7fff) as f64 / 32767.0
}

/// Save SVG to file
fn save_svg(svg: &str, path: &str) {
    fs::write(path, svg).unwrap_or_else(|_| panic!("Failed to write {}", path));
    println!("  -> Saved: {}", path);
}
