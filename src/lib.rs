use plotly::{common::Mode, Bar, Layout, Plot, Scatter};
use serde::Deserialize;
use usvg::TreeParsing;
use usvg_text_layout::TreeTextToPath;
use worker::console_log;
use worker::*;

#[derive(Deserialize)]
struct GraphRequest {
    graph_type: String,
    data: Vec<f64>,
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
}

#[event(fetch)]
pub async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    if req.method() != Method::Post {
        return Response::error("Method not allowed", 405);
    }

    let graph_req: GraphRequest = match req.json().await {
        Ok(val) => val,
        Err(_) => return Response::error("Invalid JSON", 400),
    };

    if graph_req.data.is_empty() {
        return Response::error("Data array is empty", 400);
    }

    let png_data = match create_chart(&graph_req) {
        Ok(data) => data,
        Err(e) => return Response::error(format!("Chart creation error: {}", e), 500),
    };

    let mut headers = Headers::new();
    headers.set("Content-Type", "image/png")?;

    Ok(Response::from_bytes(png_data)?.with_headers(headers))
}

fn create_chart(graph_req: &GraphRequest) -> core::result::Result<Vec<u8>, String> {
    let mut plot = Plot::new();
    let layout = Layout::new()
        .title(graph_req.title.as_deref().unwrap_or(""))
        .x_axis(plotly::layout::Axis::new().title(graph_req.x_label.as_deref().unwrap_or("")))
        .y_axis(plotly::layout::Axis::new().title(graph_req.y_label.as_deref().unwrap_or("")))
        .width(800)
        .height(600);

    match graph_req.graph_type.as_str() {
        "bar" => {
            let trace = Bar::new((0..graph_req.data.len()).collect(), graph_req.data.clone());
            plot.add_trace(trace);
        }
        "scatter" => {
            let trace = Scatter::new((0..graph_req.data.len()).collect(), graph_req.data.clone());
            plot.add_trace(trace);
        }
        _ => {
            let trace = Scatter::new((0..graph_req.data.len()).collect(), graph_req.data.clone())
                .mode(Mode::Lines);
            plot.add_trace(trace);
        }
    }

    plot.set_layout(layout);

    let max_value = graph_req
        .data
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let chart_content = match graph_req.graph_type.as_str() {
        "bar" => generate_bar_chart_svg(&graph_req.data),
        "scatter" => generate_scatter_chart_svg(&graph_req.data),
        _ => generate_line_chart_svg(&graph_req.data),
    };

    let svg_data = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600" viewBox="0 0 800 600">
    <rect width="800" height="600" fill="white"/>

    <!-- タイトル -->
    <path d="M 400 50 L 400 50" stroke="black" stroke-width="1" fill="none"/>
    <text x="400" y="50" text-anchor="middle" font-size="24" fill="black">{title}</text>

    <!-- Y軸 -->
    <line x1="100" y1="500" x2="100" y2="100" stroke="black" stroke-width="2"/>
    {y_axis_ticks}
    <path d="M 50 300 L 50 300" stroke="black" stroke-width="1" fill="none"/>
    <text x="50" y="300" text-anchor="middle" font-size="16" fill="black" transform="rotate(-90, 50, 300)">{y_label}</text>

    <!-- X軸 -->
    <line x1="100" y1="500" x2="700" y2="500" stroke="black" stroke-width="2"/>
    {x_axis_ticks}
    <path d="M 400 550 L 400 550" stroke="black" stroke-width="1" fill="none"/>
    <text x="400" y="550" text-anchor="middle" font-size="16" fill="black">{x_label}</text>

    <!-- チャート -->
    <g transform="translate(100, 100)">
        {chart_content}
    </g>
</svg>"#,
        title = graph_req.title.as_deref().unwrap_or(""),
        y_axis_ticks = generate_y_axis_ticks(max_value),
        y_label = graph_req.y_label.as_deref().unwrap_or(""),
        x_axis_ticks = generate_x_axis_ticks(graph_req.data.len()),
        x_label = graph_req.x_label.as_deref().unwrap_or(""),
        chart_content = chart_content
    );

    svg_to_png(&svg_data)
}

fn generate_y_axis_ticks(max_value: f64) -> String {
    let num_ticks = 5;
    let tick_step = max_value / num_ticks as f64;
    let mut ticks = String::new();

    for i in 0..=num_ticks {
        let y = 500.0 - (400.0 * i as f64 / num_ticks as f64);
        let value = tick_step * i as f64;
        ticks.push_str(&format!(
            r#"<line x1="95" y1="{y}" x2="100" y2="{y}" stroke="black" stroke-width="2"/>
            <path d="M 90 {} L 90 {}" stroke="black" stroke-width="1" fill="none"/>
            <text x="90" y="{}" text-anchor="end" font-size="12" fill="black">{:.1}</text>
            "#,
            y + 4.0,
            y + 4.0,
            y + 4.0,
            value
        ));
    }
    ticks
}

fn generate_x_axis_ticks(num_bars: usize) -> String {
    let mut ticks = String::new();
    let bar_width = 600.0 / num_bars as f64;

    for i in 0..num_bars {
        let x = 100.0 + (i as f64 * bar_width) + (bar_width / 2.0);
        ticks.push_str(&format!(
            r#"<line x1="{x}" y1="500" x2="{x}" y2="505" stroke="black" stroke-width="2"/>
            <path d="M {x} 520 L {x} 520" stroke="black" stroke-width="1" fill="none"/>
            <text x="{x}" y="520" text-anchor="middle" font-size="12" fill="black">{}</text>
            "#,
            i + 1
        ));
    }
    ticks
}

fn generate_bar_chart_svg(data: &[f64]) -> String {
    let max_value = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let bar_width = 600.0 / data.len() as f64;
    let scale = 400.0 / max_value;

    data.iter()
        .enumerate()
        .map(|(i, &value)| {
            let x = i as f64 * bar_width;
            let height = value * scale;
            let y = 400.0 - height;
            let text_x = x + (bar_width * 0.4);
            let text_y = y + (height / 2.0);
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="blue"/>
                <path d="M {} {} h 0" stroke="white" stroke-width="1"/>
                <path d="M {} {} h 0" stroke="black" stroke-width="1"/>
                <path d="M {} {} h 0" stroke="white" stroke-width="2" fill="none"/>
                {}"#,
                x,
                y,
                bar_width * 0.8,
                height,
                text_x,
                text_y,
                text_x,
                text_y,
                text_x,
                text_y,
                generate_value_text(text_x, text_y, value)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_line_chart_svg(data: &[f64]) -> String {
    let max_value = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let segment_width = 600.0 / (data.len() - 1) as f64;
    let scale = 400.0 / max_value;

    // パスデータを生成
    let path_data = data
        .iter()
        .enumerate()
        .map(|(i, &value)| {
            let x = i as f64 * segment_width;
            let y = 400.0 - (value * scale);
            if i == 0 {
                format!("M {} {}", x, y)
            } else {
                format!("L {} {}", x, y)
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    // データポイントとラベルを生成
    let points = data
        .iter()
        .enumerate()
        .map(|(i, &value)| {
            let x = i as f64 * segment_width;
            let y = 400.0 - (value * scale);
            format!(
                r#"<circle cx="{}" cy="{}" r="4" fill="blue"/>
            {}"#,
                x,
                y,
                generate_value_text(x, y - 15.0, value)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"<path d="{}" stroke="blue" stroke-width="2" fill="none"/>
        {}"#,
        path_data, points
    )
}

fn generate_scatter_chart_svg(data: &[f64]) -> String {
    let max_value = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let segment_width = 600.0 / data.len() as f64;
    let scale = 400.0 / max_value;

    data.iter()
        .enumerate()
        .map(|(i, &value)| {
            let x = i as f64 * segment_width;
            let y = 400.0 - (value * scale);
            format!(
                r#"<circle cx="{}" cy="{}" r="6" fill="blue"/>
                {}"#,
                x,
                y,
                generate_value_text(x, y - 15.0, value)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_value_text(x: f64, y: f64, value: f64) -> String {
    format!(
        r#"<g transform="translate({} {})">
            <path d="M 0 0 h 0" stroke="black" stroke-width="1" fill="none"/>
            <text text-anchor="middle" font-size="12" fill="black">{:.1}</text>
        </g>"#,
        x, y, value
    )
}

fn svg_to_png(svg_str: &str) -> core::result::Result<Vec<u8>, String> {
    // フォントデータベースを初期化
    let mut fontdb = fontdb::Database::new();

    // フォントデータをバイナリとして直接埋め込み
    static FONT_DATA: &[u8] = include_bytes!("../assets/kunimaru.ttf");
    let font_data = FONT_DATA.to_vec();
    fontdb.load_font_data(font_data);

    // 読み込まれたフォントの情報をログ出力
    for face in fontdb.faces() {
        if let Some(families) = face.families.first() {
            console_log!("Loaded font family: {}", families.0);
        }
    }

    // SVGパース用のオプション設定
    let opt = usvg::Options {
        font_family: fontdb
            .faces()
            .next()
            .and_then(|face| face.families.first())
            .map(|family| family.0.clone())
            .unwrap_or_else(|| "sans-serif".to_string()),
        font_size: 30.0,
        dpi: 96.0,
        ..usvg::Options::default()
    };

    // SVGをパース
    let mut tree =
        usvg::Tree::from_str(svg_str, &opt).map_err(|e| format!("Failed to parse SVG: {}", e))?;

    // テキストをパスに変換
    tree.convert_text(&fontdb);

    // resvgツリーを作成
    let rtree = resvg::Tree::from_usvg(&tree);

    // レンダリングサイズを取得
    let width = rtree.size.width() as u32;
    let height = rtree.size.height() as u32;

    // ピクスマップを作成
    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or("Failed to create pixmap")?;

    // SVGをレンダリング
    rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());

    // PNGにエンコード
    Ok(pixmap
        .encode_png()
        .map_err(|e| format!("Failed to encode PNG: {}", e))?)
}
