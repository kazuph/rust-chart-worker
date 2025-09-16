use super::Chart;
use crate::models::GraphRequest;
use crate::utils::{self, svg};
use std::f64::consts::PI;

pub struct RadarChart {}

impl Chart for RadarChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let mut svg_content = svg::create_svg_header_no_axes(
            request.title.as_deref(),
            None,
            None,
        );

        // データとラベルを保持する構造を生成
        let (series, axis_labels) = if request.series.is_empty() {
            let series_data = vec![request.data.iter().map(|&v| v).collect::<Vec<f64>>()];
            let labels: Vec<String> = request.data.iter().enumerate()
                .map(|(i, _)| format!("Axis {}", i + 1))
                .collect();
            (series_data, labels)
        } else {
            let labels = request.series[0]
                .data
                .iter()
                .map(|d| d.label.as_ref().unwrap_or(&"".to_string()).to_string())
                .collect();
                
            let series_data = request
                .series
                .iter()
                .map(|s| s.data.iter().map(|d| d.value).collect::<Vec<f64>>())
                .collect();
                
            (series_data, labels)
        };

        let max_value = if request.series.is_empty() {
            request.data.iter().copied().fold(0.0, f64::max)
        } else {
            super::get_max_value(&request.series)
        };
        let center_x = 400.0;
        let center_y = 300.0;
        let radius = 200.0;
        let num_axes = series.first().map(|s| s.len()).unwrap_or(6);
        let angle_step = 2.0 * PI / num_axes as f64;

        // Draw background circles
        for i in 1..=5 {
            let r = radius * (i as f64 / 5.0);
            let mut points = Vec::new();
            for j in 0..num_axes {
                let angle = -PI / 2.0 + j as f64 * angle_step;
                let x = center_x + r * angle.cos();
                let y = center_y + r * angle.sin();
                points.push((x, y));
            }
            let path = points
                .iter()
                .enumerate()
                .map(|(i, (x, y))| {
                    if i == 0 {
                        format!("M {:.1} {:.1}", x, y)
                    } else {
                        format!("L {:.1} {:.1}", x, y)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            svg_content.push_str(&format!(
                r#"<path d="{} Z" style="stroke:#CCCCCC;stroke-width:1;fill:none" />"#,
                path
            ));
        }

        // Draw axis lines and labels
        for i in 0..num_axes {
            let angle = -PI / 2.0 + i as f64 * angle_step;
            let end_x = center_x + radius * angle.cos();
            let end_y = center_y + radius * angle.sin();

            // Draw axis line
            svg_content.push_str(&format!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="stroke:#CCCCCC;stroke-width:1" />"#,
                center_x, center_y, end_x, end_y
            ));

            // Draw axis label
            let label_x = center_x + (radius + 20.0) * angle.cos();
            let label_y = center_y + (radius + 20.0) * angle.sin();
            let empty_string = String::new();
            let label = axis_labels.get(i).unwrap_or(&empty_string);
            svg_content.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" font-size="12">{}</text>"#,
                label_x,
                label_y,
                label
            ));
        }

        // Draw data
        for (series_idx, series_data) in series.iter().enumerate() {
            let color = if let Some(series) = request.series.get(series_idx) {
                series.color.as_deref().unwrap_or_else(|| {
                    request.colors.as_ref()
                        .and_then(|colors| colors.get(series_idx))
                        .map(String::as_str)
                        .unwrap_or_else(|| utils::get_default_colors()[series_idx % utils::get_default_colors().len()])
                })
            } else {
                utils::get_default_colors()[series_idx % utils::get_default_colors().len()]
            };

            let mut points = Vec::new();
            for (i, &value) in series_data.iter().enumerate() {
                let angle = -PI / 2.0 + i as f64 * angle_step;
                let r = radius * (value / max_value);
                let x = center_x + r * angle.cos();
                let y = center_y + r * angle.sin();
                points.push((x, y));
            }

            // Draw polygon
            let path = points
                .iter()
                .enumerate()
                .map(|(i, (x, y))| {
                    if i == 0 {
                        format!("M {:.1} {:.1}", x, y)
                    } else {
                        format!("L {:.1} {:.1}", x, y)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            svg_content.push_str(&format!(
                r#"<path d="{} Z" style="stroke:{};stroke-width:2;fill:{};fill-opacity:0.2" />"#,
                path, color, color
            ));

            // Draw points and values
            for (_i, ((x, y), &value)) in points.iter().zip(series_data.iter()).enumerate() {
                svg_content.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="4" fill="{}" />"#,
                    x, y, color
                ));
                svg_content.push_str(&utils::svg::generate_value_text(*x, *y, value));
            }
        }

        if !request.series.is_empty() {
            svg_content.push_str(&svg::create_legend(&request.series, 660.0, 50.0));
        }

        svg_content.push_str(svg::create_svg_footer());
        svg_content
    }
}
