mod charts;
mod models;
mod utils;

use models::GraphRequest;
use worker::*;

#[event(fetch)]
pub async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;

    let result = match req.method() {
        Method::Get => {
            let graph_req = match parse_query_params(url) {
                Ok(req) => req,
                Err(e) => return Response::error(e, 400),
            };

            if graph_req.data.is_empty() && graph_req.series.is_empty() {
                return Response::error("No data provided", 400);
            }

            let chart = charts::create_chart(&graph_req);
            let svg_content = chart.generate(&graph_req);

            // フォントデータベースを初期化
            let mut fontdb = fontdb::Database::new();
            static FONT_DATA: &[u8] = include_bytes!("../assets/MPLUS1p-Regular.ttf");
            fontdb.load_font_data(FONT_DATA.to_vec());

            // SVGパース用のオプション設定
            let opt = usvg::Options {
                font_family: "M PLUS 1p".to_string(),
                font_size: 12.0,
                dpi: 96.0,
                ..usvg::Options::default()
            };

            let png_data = match utils::png::svg_to_png(&svg_content) {
                Ok(data) => data,
                Err(e) => return Response::error(format!("PNG conversion error: {}", e), 500),
            };

            let mut headers = Headers::new();
            headers.set("Content-Type", "image/png")?;
            headers.set("Cache-Control", "public, max-age=86400")?;
            headers.set("Access-Control-Allow-Origin", "*")?;

            let resp = Response::from_bytes(png_data)?;
            Ok(resp.with_headers(headers))
        }
        Method::Options => {
            let mut headers = Headers::new();
            headers.set("Access-Control-Allow-Origin", "*")?;
            headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
            let resp = Response::empty()?;
            Ok(resp.with_headers(headers))
        }
        _ => Response::error("Method not allowed", 405),
    };

    result
}

fn parse_query_params(url: Url) -> core::result::Result<GraphRequest, &'static str> {
    let params = url.query_pairs();
    let mut graph_type = models::GraphType::default();
    let mut data: Vec<f64> = Vec::new();
    let mut series: Vec<models::Series> = Vec::new();
    let mut title: Option<String> = None;
    let mut x_label: Option<String> = None;
    let mut y_label: Option<String> = None;
    let mut colors: Option<Vec<String>> = None;

    // シリーズデータのための一時的な保存領域
    let mut series_values: Vec<f64> = Vec::new();
    let mut series_labels: Vec<String> = Vec::new();

    for (key, value) in params {
        match key.as_ref() {
            "type" => {
                graph_type = match value.as_ref() {
                    "bar" => models::GraphType::Bar,
                    "scatter" => models::GraphType::Scatter,
                    "pie" => models::GraphType::Pie,
                    "donut" => models::GraphType::Donut,
                    "area" => models::GraphType::Area,
                    "radar" => models::GraphType::Radar,
                    _ => models::GraphType::Line,
                };
            }
            "data" => {
                data = value
                    .split(',')
                    .filter_map(|s| s.parse::<f64>().ok())
                    .collect();
                series_values = data.clone();
            }
            "labels" => {
                series_labels = value.split(',').map(String::from).collect();
            }
            "title" => title = Some(value.into_owned()),
            "x_label" => x_label = Some(value.into_owned()),
            "y_label" => y_label = Some(value.into_owned()),
            "colors" => {
                colors = Some(value.split(',').map(String::from).collect());
            }
            _ => {}
        }
    }

    // シリーズデータを構築
    if !series_values.is_empty() {
        let mut series_data = Vec::new();
        for (i, &value) in series_values.iter().enumerate() {
            let label = series_labels.get(i).cloned();
            let color = colors.as_ref().and_then(|c| c.get(i).cloned());
            series_data.push(models::DataPoint {
                value,
                label,
                color,
            });
        }
        series.push(models::Series {
            name: None,
            data: series_data,
            color: colors.as_ref().and_then(|c| c.first().cloned()),
        });
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
