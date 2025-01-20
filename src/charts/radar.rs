use super::Chart;
use crate::models::GraphRequest;
use crate::utils::{self, svg};
use std::f64::consts::PI;

pub struct RadarChart {}

impl Chart for RadarChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let mut svg_content = svg::create_svg_header(
            request.title.as_deref(),
            request.x_label.as_deref(),
            request.y_label.as_deref(),
        );

        let series = if request.series.is_empty() {
            vec![request.data.iter().map(|&v| v).collect::<Vec<f64>>()]
        } else {
            request
                .series
                .iter()
                .map(|s| s.data.iter().map(|d| d.value).collect::<Vec<f64>>())
                .collect()
        };

        let max_value = super::get_max_value(&request.series);
        let center_x = 400.0;
        let center_y = 300.0;
        let radius = 200.0;
        let num_axes = series[0].len();
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
            svg_content.push_str(&format!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="stroke:#CCCCCC;stroke-width:1" />"#,
                center_x, center_y, end_x, end_y
            ));

            // Draw axis label
            let label_x = center_x + (radius + 20.0) * angle.cos();
            let label_y = center_y + (radius + 20.0) * angle.sin();
            svg_content.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" font-size="12">{}</text>"#,
                label_x,
                label_y,
                i + 1
            ));
        }

        // Draw data
        for (series_idx, series_data) in series.iter().enumerate() {
            let color = request
                .colors
                .as_ref()
                .and_then(|c| c.get(series_idx))
                .map(String::as_str)
                .unwrap_or(
                    utils::get_default_colors()[series_idx % utils::get_default_colors().len()],
                );

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
            for (i, ((x, y), &value)) in points.iter().zip(series_data.iter()).enumerate() {
                svg_content.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="4" fill="{}" />"#,
                    x, y, color
                ));
                svg_content.push_str(&utils::svg::generate_value_text(*x, *y, value));
            }
        }

        if !request.series.is_empty() {
            svg_content.push_str(&svg::create_legend(&request.series, 520.0, 50.0));
        }

        svg_content.push_str(svg::create_svg_footer());
        svg_content
    }
}
