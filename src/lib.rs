use plotly::{common::Mode, Bar, Layout, Plot, Scatter};
use serde::Deserialize;
use usvg::TreeParsing;
use usvg_text_layout::TreeTextToPath;
use worker::console_log;
use worker::*;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphType {
    Line,
    Bar,
    Scatter,
    Pie,
    Donut,
    Area,
    Radar,
}

impl Default for GraphType {
    fn default() -> Self {
        GraphType::Line
    }
}

#[derive(Deserialize, Clone)]
pub struct DataPoint {
    value: f64,
    label: Option<String>,
    color: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Series {
    #[allow(dead_code)]
    name: Option<String>,
    data: Vec<DataPoint>,
    color: Option<String>,
}

#[derive(Deserialize)]
struct GraphRequest {
    #[serde(default)]
    graph_type: GraphType,
    #[serde(default)]
    series: Vec<Series>,
    #[serde(default)]
    data: Vec<f64>, // 後方互換性のため残す
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
    colors: Option<Vec<String>>, // Add colors field
}

#[event(fetch)]
pub async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let graph_req = match req.method() {
        Method::Post => match req.json().await {
            Ok(val) => val,
            Err(_) => return Response::error("Invalid JSON", 400),
        },
        Method::Get => match parse_query_params(req.url()?) {
            Ok(val) => val,
            Err(e) => return Response::error(e, 400),
        },
        _ => return Response::error("Method not allowed", 405),
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

fn parse_query_params(url: Url) -> core::result::Result<GraphRequest, &'static str> {
    let params = url.query_pairs();
    let mut graph_type = GraphType::default();
    let mut data: Vec<f64> = Vec::new();
    let mut series: Vec<Series> = Vec::new();
    let mut title: Option<String> = None;
    let mut x_label: Option<String> = None;
    let mut y_label: Option<String> = None;
    let mut colors: Option<Vec<String>> = None;

    // シリーズデータのための一時的な保存領域
    let mut series_values: Vec<f64> = Vec::new();
    let mut series_labels: Vec<String> = Vec::new();
    let mut series_colors: Vec<String> = Vec::new();

    for (key, value) in params {
        match key.as_ref() {
            "type" => {
                graph_type = match value.as_ref() {
                    "bar" => GraphType::Bar,
                    "scatter" => GraphType::Scatter,
                    "pie" => GraphType::Pie,
                    "donut" => GraphType::Donut,
                    "area" => GraphType::Area,
                    "radar" => GraphType::Radar,
                    _ => GraphType::Line,
                };
            }
            "data" => {
                data = value
                    .split(',')
                    .filter_map(|s| s.trim().parse::<f64>().ok())
                    .collect();
                // データをシリーズデータとしても保存
                series_values = data.clone();
            }
            "labels" => {
                series_labels = value.split(',').map(|s| s.trim().to_string()).collect();
            }
            "colors" => {
                colors = Some(value.split(',').map(|s| s.trim().to_string()).collect());
            }
            "title" => title = Some(value.to_string()),
            "x_label" => x_label = Some(value.to_string()),
            "y_label" => y_label = Some(value.to_string()),
            _ => {}
        }
    }

    // シリーズデータの構築
    if !series_values.is_empty() {
        let mut series_data = Vec::new();
        for (i, &value) in series_values.iter().enumerate() {
            let label = series_labels.get(i).cloned();
            let color = colors.as_ref().and_then(|c| c.get(i).cloned());
            series_data.push(DataPoint {
                value,
                label,
                color,
            });
        }
        series.push(Series {
            name: None,
            data: series_data,
            color: colors.as_ref().and_then(|c| c.first().cloned()),
        });
    }

    if data.is_empty() && series.is_empty() {
        return Err("No data provided or invalid data format");
    }

    Ok(GraphRequest {
        graph_type,
        series,
        data,
        title,
        x_label,
        y_label,
        colors,
    })
}

fn get_default_colors() -> Vec<&'static str> {
    vec![
        "#0000FF", "#FFB3B3", "#B3E0FF", "#FFE6B3", "#B3FFB3", "#E6B3FF", "#FFD9B3",
    ]
}

fn create_chart(graph_req: &GraphRequest) -> core::result::Result<Vec<u8>, String> {
    let mut plot = Plot::new();
    let layout = Layout::new()
        .title(graph_req.title.as_deref().unwrap_or(""))
        .x_axis(plotly::layout::Axis::new().title(graph_req.x_label.as_deref().unwrap_or("")))
        .y_axis(plotly::layout::Axis::new().title(graph_req.y_label.as_deref().unwrap_or("")))
        .width(800)
        .height(600);

    // 後方互換性のためのデータ変換
    let series = if graph_req.series.is_empty() && !graph_req.data.is_empty() {
        vec![Series {
            name: None,
            data: graph_req
                .data
                .iter()
                .enumerate()
                .map(|(i, &v)| DataPoint {
                    value: v,
                    label: None,
                    color: graph_req.colors.as_ref().and_then(|c| c.get(i).cloned()),
                })
                .collect(),
            color: graph_req.colors.as_ref().and_then(|c| c.first().cloned()),
        }]
    } else {
        graph_req.series.clone()
    };

    let (chart_content, needs_axes) = match graph_req.graph_type {
        GraphType::Bar => (
            generate_bar_chart_svg(
                &graph_req.data,
                &series[0]
                    .data
                    .iter()
                    .filter_map(|d| d.color.clone())
                    .collect::<Vec<String>>(),
            ),
            true,
        ),
        GraphType::Scatter => (
            generate_scatter_chart_svg(
                &graph_req.data,
                &series[0]
                    .data
                    .iter()
                    .filter_map(|d| d.color.clone())
                    .collect::<Vec<String>>(),
            ),
            true,
        ),
        GraphType::Line => (
            generate_line_chart_svg(
                &graph_req.data,
                &series[0]
                    .data
                    .iter()
                    .filter_map(|d| d.color.clone())
                    .collect::<Vec<String>>(),
            ),
            true,
        ),
        GraphType::Pie => (generate_pie_chart_svg(&series, false), false),
        GraphType::Donut => (generate_pie_chart_svg(&series, true), false),
        GraphType::Area => (generate_area_chart_svg(&series), true),
        GraphType::Radar => (generate_radar_chart_svg(&series), false),
    };

    let max_value = if needs_axes {
        if !graph_req.data.is_empty() {
            graph_req
                .data
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max)
        } else {
            series
                .iter()
                .flat_map(|s| s.data.iter())
                .map(|d| d.value)
                .fold(f64::NEG_INFINITY, f64::max)
        }
    } else {
        0.0
    };

    let (transform_x, transform_y) = if needs_axes {
        (100, 100)
    } else {
        (400, 300) // 中心位置を変更
    };

    let svg_data = if needs_axes {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600" viewBox="0 0 800 600">
    <rect width="800" height="600" fill="white"/>

    <!-- タイトル -->
    <path d="M 400 50 L 400 50" stroke="black" stroke-width="1" fill="none"/>
    <text x="400" y="50" text-anchor="middle" font-size="24" fill="black">{title}</text>

    <!-- チャート -->
    <g transform="translate({transform_x}, {transform_y})">
        {chart_content}
    </g>

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
</svg>"#,
            title = graph_req.title.as_deref().unwrap_or(""),
            y_axis_ticks = generate_y_axis_ticks(max_value),
            y_label = graph_req.y_label.as_deref().unwrap_or(""),
            x_axis_ticks = generate_x_axis_ticks(if !graph_req.data.is_empty() {
                graph_req.data.len()
            } else {
                series[0].data.len()
            }),
            x_label = graph_req.x_label.as_deref().unwrap_or(""),
            transform_x = transform_x,
            transform_y = transform_y,
            chart_content = chart_content
        )
    } else {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600" viewBox="0 0 800 600">
    <rect width="800" height="600" fill="white"/>
    <!-- タイトル -->
    <text x="400" y="50" text-anchor="middle" font-size="24" fill="black">{title}</text>
    <g transform="translate({transform_x}, {transform_y})">
        {chart_content}
    </g>
</svg>"#,
            title = graph_req.title.as_deref().unwrap_or(""),
            transform_x = transform_x,
            transform_y = transform_y,
            chart_content = chart_content
        )
    };

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

fn generate_bar_chart_svg(data: &[f64], colors: &[String]) -> String {
    let max_value = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let bar_width = 600.0 / data.len() as f64;
    let scale = 400.0 / max_value;
    let default_colors = get_default_colors();
    let color = colors
        .first()
        .map(|s| s.as_str())
        .unwrap_or(default_colors[0]);

    data.iter()
        .enumerate()
        .map(|(i, &value)| {
            let x = i as f64 * bar_width;
            let height = value * scale;
            let y = 400.0 - height;
            let text_x = x + (bar_width * 0.4);
            let text_y = y + (height / 2.0);
            format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/><path d=\"M {} {} h 0\" stroke=\"white\" stroke-width=\"1\"/><path d=\"M {} {} h 0\" stroke=\"black\" stroke-width=\"1\"/><path d=\"M {} {} h 0\" stroke=\"white\" stroke-width=\"2\" fill=\"none\"/>{}>",
                x,
                y,
                bar_width * 0.8,
                height,
                color,
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

fn generate_line_chart_svg(data: &[f64], colors: &[String]) -> String {
    let max_value = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let segment_width = 600.0 / (data.len() - 1) as f64;
    let scale = 400.0 / max_value;
    let default_colors = get_default_colors();
    let color = colors
        .get(0)
        .map(|s| s.as_str())
        .unwrap_or(default_colors[0]);

    let mut path = String::new();
    let mut points = String::new();

    // Generate path
    path.push_str(&format!("<path d=\"M"));
    for (i, &value) in data.iter().enumerate() {
        let x = i as f64 * segment_width;
        let y = 400.0 - (value * scale);
        if i == 0 {
            path.push_str(&format!(" {} {}", x, y));
        } else {
            path.push_str(&format!(" L {} {}", x, y));
        }
    }
    path.push_str(&format!(
        "\" stroke=\"{}\" stroke-width=\"2\" fill=\"none\"/>",
        color
    ));

    // Generate points
    for (i, &value) in data.iter().enumerate() {
        let x = i as f64 * segment_width;
        let y = 400.0 - (value * scale);
        points.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\"/>{}>",
            x,
            y,
            color,
            generate_value_text(x, y - 15.0, value)
        ));
    }

    format!("{}\n{}", path, points)
}

fn generate_scatter_chart_svg(data: &[f64], colors: &[String]) -> String {
    let max_value = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let segment_width = 600.0 / data.len() as f64;
    let scale = 400.0 / max_value;
    let default_colors = get_default_colors();
    let color = colors
        .first()
        .map(|s| s.as_str())
        .unwrap_or(default_colors[0]);

    data.iter()
        .enumerate()
        .map(|(i, &value)| {
            let x = i as f64 * segment_width;
            let y = 400.0 - (value * scale);
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"6\" fill=\"{}\"/>{}>",
                x,
                y,
                color,
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

fn generate_pie_chart_svg(series: &[Series], is_donut: bool) -> String {
    let total: f64 = series
        .iter()
        .flat_map(|s| s.data.iter())
        .map(|d| d.value)
        .sum();

    let mut current_angle = 0.0;
    let center_x = 0.0; // 中心を(0,0)に変更
    let center_y = 0.0;
    let radius = 180.0;
    let inner_radius = if is_donut { radius * 0.6 } else { 0.0 };
    let colors = vec![
        "#FFB3B3", "#B3E0FF", "#FFE6B3", "#B3FFB3", "#E6B3FF", "#FFD9B3",
    ];

    let mut paths = String::new();
    let mut labels = String::new();

    for (series_idx, series) in series.iter().enumerate() {
        for (idx, data) in series.data.iter().enumerate() {
            let percentage = data.value / total;
            let angle = percentage * 360.0;
            let end_angle = current_angle + angle;

            let color = data
                .color
                .as_deref()
                .or(series.color.as_deref())
                .unwrap_or_else(|| colors[idx % colors.len()]);

            // 円弧のパスを生成
            let start_rad = current_angle.to_radians();
            let end_rad = end_angle.to_radians();

            let start_x = center_x + radius * start_rad.cos();
            let start_y = center_y + radius * start_rad.sin();
            let end_x = center_x + radius * end_rad.cos();
            let end_y = center_y + radius * end_rad.sin();

            let large_arc = if angle > 180.0 { 1 } else { 0 };

            if is_donut {
                let inner_start_x = center_x + inner_radius * start_rad.cos();
                let inner_start_y = center_y + inner_radius * start_rad.sin();
                let inner_end_x = center_x + inner_radius * end_rad.cos();
                let inner_end_y = center_y + inner_radius * end_rad.sin();

                paths.push_str(&format!(
                    r#"<path d="M {} {} A {} {} 0 {} 1 {} {} L {} {} A {} {} 0 {} 0 {} {} Z" fill="{}" stroke="white"/>"#,
                    start_x, start_y, radius, radius, large_arc, end_x, end_y,
                    inner_end_x, inner_end_y, inner_radius, inner_radius, large_arc, inner_start_x, inner_start_y,
                    color
                ));
            } else {
                paths.push_str(&format!(
                    r#"<path d="M {} {} A {} {} 0 {} 1 {} {} L {} {} Z" fill="{}" stroke="white"/>"#,
                    start_x, start_y, radius, radius, large_arc, end_x, end_y, center_x, center_y,
                    color
                ));
            }

            // ラベルの位置を計算
            let label_angle = (current_angle + angle / 2.0).to_radians();
            let label_radius = radius * 1.2;
            let label_x = center_x + label_radius * label_angle.cos();
            let label_y = center_y + label_radius * label_angle.sin();

            let label_text = data.label.as_deref().unwrap_or_else(|| "");
            let value_text = format!("{:.1}%", percentage * 100.0);

            labels.push_str(&format!(
                r#"<g transform="translate({} {})">
                    <text text-anchor="middle" font-size="12" fill="black">{}</text>
                    <text text-anchor="middle" font-size="12" fill="black" dy="14">{}</text>
                </g>"#,
                label_x, label_y, label_text, value_text
            ));

            current_angle = end_angle;
        }
    }

    format!(r#"<g transform="translate(0, 0)">{}{}</g>"#, paths, labels)
}

fn generate_area_chart_svg(series: &[Series]) -> String {
    let max_value = series
        .iter()
        .flat_map(|s| s.data.iter())
        .map(|d| d.value)
        .fold(f64::NEG_INFINITY, f64::max);

    let colors = vec![
        "#FFB3B3", "#B3E0FF", "#FFE6B3", "#B3FFB3", "#E6B3FF", "#FFD9B3",
    ];
    let mut paths = String::new();

    for (series_idx, series) in series.iter().enumerate() {
        let data_len = series.data.len();
        if data_len == 0 {
            continue;
        }

        let segment_width = 600.0 / (data_len - 1) as f64;
        let scale = 400.0 / max_value;
        let color = series
            .color
            .as_deref()
            .unwrap_or_else(|| colors[series_idx % colors.len()]);

        // エリアの塗りつぶし部分のパス
        let mut area_path = format!("M {} {} ", 0.0, 400.0 - (series.data[0].value * scale));

        // 上部のライン
        for (i, data) in series.data.iter().enumerate().skip(1) {
            let x = i as f64 * segment_width;
            let y = 400.0 - (data.value * scale);
            area_path.push_str(&format!("L {} {} ", x, y));
        }

        // 下部のライン（ベースライン）
        for i in (0..data_len).rev() {
            let x = i as f64 * segment_width;
            area_path.push_str(&format!("L {} {} ", x, 400.0));
        }

        area_path.push_str("Z");

        paths.push_str(&format!(
            r#"<path d="{}" fill="{}" fill-opacity="0.2" stroke="{}" stroke-width="2"/>"#,
            area_path, color, color
        ));

        // データポイントとラベル
        for (i, data) in series.data.iter().enumerate() {
            let x = i as f64 * segment_width;
            let y = 400.0 - (data.value * scale);
            paths.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"#0000FF\"/>{}>",
                x,
                y,
                generate_value_text(x, y - 15.0, data.value)
            ));
        }
    }

    paths
}

fn generate_radar_chart_svg(series: &[Series]) -> String {
    let sides = series.iter().map(|s| s.data.len()).max().unwrap_or(0);
    if sides < 3 {
        return String::new();
    }

    let max_value = series
        .iter()
        .flat_map(|s| s.data.iter())
        .map(|d| d.value)
        .fold(f64::NEG_INFINITY, f64::max);

    let center_x = 0.0; // 中心を(0,0)に変更
    let center_y = 0.0;
    let radius = 180.0;
    let colors = vec![
        "#FFB3B3", "#B3E0FF", "#FFE6B3", "#B3FFB3", "#E6B3FF", "#FFD9B3",
    ];

    let mut svg = String::new();

    // 背景の多角形を描画
    for i in 1..=5 {
        let ratio = i as f64 * 0.2;
        let mut points = Vec::new();
        for j in 0..sides {
            let angle = (360.0 / sides as f64 * j as f64).to_radians();
            let x = center_x + radius * ratio * angle.cos();
            let y = center_y + radius * ratio * angle.sin();
            points.push(format!("{},{}", x, y));
        }
        svg.push_str(&format!(
            "<polygon points=\"{}\" fill=\"none\" stroke=\"#ddd\" stroke-width=\"1\"/>",
            points.join(" ")
        ));
    }

    // 軸線を描画
    for i in 0..sides {
        let angle = (360.0 / sides as f64 * i as f64).to_radians();
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#ddd\" stroke-width=\"1\"/>",
            center_x, center_y, x, y
        ));
    }

    // データを描画
    for (series_idx, series) in series.iter().enumerate() {
        let color = series
            .color
            .as_deref()
            .unwrap_or_else(|| colors[series_idx % colors.len()]);
        let mut points = Vec::new();
        let mut labels = String::new();

        for (i, data) in series.data.iter().enumerate() {
            let ratio = data.value / max_value;
            let angle = (360.0 / sides as f64 * i as f64).to_radians();
            let x = center_x + radius * ratio * angle.cos();
            let y = center_y + radius * ratio * angle.sin();
            points.push(format!("{},{}", x, y));

            // ラベルを追加
            let label_x = center_x + (radius + 20.0) * angle.cos();
            let label_y = center_y + (radius + 20.0) * angle.sin();
            let label = data.label.as_deref().unwrap_or("");
            labels.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12\">{}</text>",
                label_x, label_y, label
            ));
        }

        svg.push_str(&format!(
            "<polygon points=\"{}\" fill=\"{}\" fill-opacity=\"0.2\" stroke=\"{}\" stroke-width=\"2\"/>",
            points.join(" "),
            color,
            color
        ));

        svg.push_str(&labels);
    }

    svg
}

fn svg_to_png(svg_str: &str) -> core::result::Result<Vec<u8>, String> {
    // フォントデータベースを初期化
    let mut fontdb = fontdb::Database::new();

    // フォントデータをバイナリとして直接埋め込み
    static FONT_DATA: &[u8] = include_bytes!("../assets/MPLUS1p-Regular.ttf");
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
