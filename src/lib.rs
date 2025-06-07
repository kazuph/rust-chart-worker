mod charts;
mod models;
mod utils;
mod themes;

use models::GraphRequest;
use worker::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_time_series_charts() {
        let themes = vec!["light", "dark"];
        
        for theme in themes {
            let request = models::GraphRequest {
                graph_type: models::GraphType::Line,
                title: Some(format!("株価推移 - 時系列データ ({})", theme)),
                x_label: Some("2025年1月-6月".to_string()),
                y_label: Some("株価（千円）".to_string()),
                data: vec![2800.0, 2950.0, 2750.0, 3100.0, 3350.0, 3200.0],
                series: vec![
                    models::Series {
                        name: Some("日経平均".to_string()),
                        data: vec![
                            models::DataPoint { value: 2800.0, label: Some("1月".to_string()), color: None },
                            models::DataPoint { value: 2950.0, label: Some("2月".to_string()), color: None },
                            models::DataPoint { value: 2750.0, label: Some("3月".to_string()), color: None },
                            models::DataPoint { value: 3100.0, label: Some("4月".to_string()), color: None },
                            models::DataPoint { value: 3350.0, label: Some("5月".to_string()), color: None },
                            models::DataPoint { value: 3200.0, label: Some("6月".to_string()), color: None },
                        ],
                        color: None,
                    },
                    models::Series {
                        name: Some("TOPIX".to_string()),
                        data: vec![
                            models::DataPoint { value: 1950.0, label: None, color: None },
                            models::DataPoint { value: 2100.0, label: None, color: None },
                            models::DataPoint { value: 1900.0, label: None, color: None },
                            models::DataPoint { value: 2250.0, label: None, color: None },
                            models::DataPoint { value: 2400.0, label: None, color: None },
                            models::DataPoint { value: 2300.0, label: None, color: None },
                        ],
                        color: None,
                    },
                ],
                theme: Some(theme.to_string()),
                colors: None,
                bins: None,
                show_density: None,
                cell_size: None,
                show_values: None,
            };

            let chart = charts::line::LineChart {};
            let svg = charts::Chart::generate(&chart, &request);
            
            let filename = format!("/data/data/com.termux/files/home/storage/shared/obsidian/attachment/timeseries_line_{}.svg", theme);
            std::fs::write(&filename, svg).expect("Failed to write SVG file");
            println!("Generated time series chart: {}", filename);
        }
    }

    #[test]
    fn test_fixed_themes() {
        let themes = vec!["light", "dark", "material", "minimal"];
        
        for theme in themes {
            let request = models::GraphRequest {
                graph_type: models::GraphType::Bar,
                title: Some(format!("修正済みバーチャート ({})", theme)),
                x_label: Some("X軸".to_string()),
                y_label: Some("Y軸".to_string()),
                data: vec![10.0, 20.0, 30.0, 25.0, 15.0],
                series: vec![
                    models::Series {
                        name: Some("売上".to_string()),
                        data: vec![
                            models::DataPoint { value: 10.0, label: None, color: None },
                            models::DataPoint { value: 20.0, label: None, color: None },
                            models::DataPoint { value: 30.0, label: None, color: None },
                            models::DataPoint { value: 25.0, label: None, color: None },
                            models::DataPoint { value: 15.0, label: None, color: None },
                        ],
                        color: None,
                    },
                    models::Series {
                        name: Some("利益".to_string()),
                        data: vec![
                            models::DataPoint { value: 5.0, label: None, color: None },
                            models::DataPoint { value: 15.0, label: None, color: None },
                            models::DataPoint { value: 20.0, label: None, color: None },
                            models::DataPoint { value: 30.0, label: None, color: None },
                            models::DataPoint { value: 25.0, label: None, color: None },
                        ],
                        color: None,
                    },
                ],
                theme: Some(theme.to_string()),
                colors: None,
                bins: None,
                show_density: None,
                cell_size: None,
                show_values: None,
            };

            let chart = charts::bar::BarChart {};
            let svg = charts::Chart::generate(&chart, &request);
            
            let filename = format!("/data/data/com.termux/files/home/storage/shared/obsidian/attachment/fixed_bar_{}.svg", theme);
            std::fs::write(&filename, svg).expect("Failed to write SVG file");
            println!("Generated: {}", filename);
        }
    }

    #[test]
    fn test_chart_generation() {
        let test_data = vec![10.0, 20.0, 30.0, 25.0, 15.0];
        let test_labels = vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string()];
        
        let data_points: Vec<models::DataPoint> = test_data
            .iter()
            .zip(test_labels.iter())
            .map(|(&value, label)| models::DataPoint {
                value,
                label: Some(label.clone()),
                color: None,
            })
            .collect();
        
        // 複数シリーズのテストデータ
        let series = vec![
            models::Series {
                name: Some("Series 1".to_string()),
                data: data_points.clone(),
                color: None,
            },
            models::Series {
                name: Some("Series 2".to_string()),
                data: vec![5.0, 15.0, 20.0, 30.0, 25.0].iter().zip(test_labels.iter()).map(|(&value, label)| {
                    models::DataPoint {
                        value,
                        label: Some(label.clone()),
                        color: None,
                    }
                }).collect(),
                color: None,
            },
            models::Series {
                name: Some("Series 3".to_string()),
                data: vec![8.0, 12.0, 25.0, 20.0, 10.0].iter().zip(test_labels.iter()).map(|(&value, label)| {
                    models::DataPoint {
                        value,
                        label: Some(label.clone()),
                        color: None,
                    }
                }).collect(),
                color: None,
            },
        ];
        
        let chart_types = vec![
            ("bar", models::GraphType::Bar),
            ("line", models::GraphType::Line),
            ("pie", models::GraphType::Pie),
            ("area", models::GraphType::Area),
            ("scatter", models::GraphType::Scatter),
            ("radar", models::GraphType::Radar),
            ("histogram", models::GraphType::Histogram),
            ("heatmap", models::GraphType::Heatmap),
            ("candlestick", models::GraphType::Candlestick),
            ("gauge", models::GraphType::Gauge),
            ("bubble", models::GraphType::Bubble),
            ("stacked_bar", models::GraphType::StackedBar),
            ("multi_line", models::GraphType::MultiLine),
        ];
        
        let themes = vec!["light", "dark", "material", "minimal"];
        
        for (name, graph_type) in chart_types {
            for theme in &themes {
                let request = models::GraphRequest {
                    graph_type,
                    series: series.clone(),
                    data: test_data.clone(),
                    title: Some(format!("Test {} Chart ({})", name.to_uppercase(), theme)),
                    x_label: Some("X Axis".to_string()),
                    y_label: Some("Y Axis".to_string()),
                    colors: Some(vec!["#3498db".to_string(), "#e74c3c".to_string(), "#2ecc71".to_string()]),
                    theme: Some(theme.to_string()),
                    bins: Some(10),
                    show_density: Some(false),
                    cell_size: Some(30.0),
                    show_values: Some(true),
                };
            
                let chart = charts::create_chart(&request);
                let mut svg_content = chart.generate(&request);
                
                // テーマを適用
                if let Some(theme_name) = &request.theme {
                    let theme_obj = themes::ThemeManager::from_name(theme_name);
                    svg_content = theme_obj.apply_to_svg(&svg_content);
                }
                
                println!("✓ {} Chart ({}) generated successfully ({} chars)", name.to_uppercase(), theme, svg_content.len());
                
                // Save SVG to Obsidian attachment folder
                let file_path = format!("/data/data/com.termux/files/home/storage/shared/obsidian/attachment/rust_chart_{}_{}.svg", name, theme);
                std::fs::write(&file_path, &svg_content).expect("Failed to write SVG file");
                println!("  - Saved to: {}", file_path);
                
                assert!(svg_content.contains("<svg"));
                assert!(svg_content.contains("</svg>"));
                assert!(svg_content.contains(request.title.as_ref().unwrap()));
            }
        }
    }
}

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
            let mut svg_content = chart.generate(&graph_req);
            
            // テーマを適用
            if let Some(theme_name) = &graph_req.theme {
                let theme = themes::ThemeManager::from_name(theme_name);
                svg_content = theme.apply_to_svg(&svg_content);
            }

            let png_data = match utils::png::svg_to_png(&svg_content) {
                Ok(data) => data,
                Err(e) => return Response::error(format!("PNG conversion error: {}", e), 500),
            };

            let mut headers = Headers::new();
            headers.set("Content-Type", "image/png")?;
            headers.set("Cache-Control", "public, max-age=604800")?; // 7日間のキャッシュ
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
    let mut theme: Option<String> = None;
    let mut bins: Option<usize> = None;
    let mut show_density: Option<bool> = None;
    let mut cell_size: Option<f64> = None;
    let mut show_values: Option<bool> = None;

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
                    "histogram" => models::GraphType::Histogram,
                    "heatmap" => models::GraphType::Heatmap,
                    "candlestick" => models::GraphType::Candlestick,
                    "gauge" => models::GraphType::Gauge,
                    "bubble" => models::GraphType::Bubble,
                    "stacked_bar" | "stackedbar" => models::GraphType::StackedBar,
                    "multi_line" | "multiline" => models::GraphType::MultiLine,
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
            "theme" => theme = Some(value.into_owned()),
            "bins" => bins = value.parse().ok(),
            "show_density" => show_density = value.parse().ok(),
            "cell_size" => cell_size = value.parse().ok(),
            "show_values" => show_values = value.parse().ok(),
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
        theme,
        bins,
        show_density,
        cell_size,
        show_values,
    })
}
