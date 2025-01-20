use super::Chart;
use crate::models::GraphRequest;
use crate::utils::{self, svg};
use std::f64::consts::PI;

pub struct PieChart {
    pub is_donut: bool,
}

impl Chart for PieChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let mut svg_content = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
<rect width="800" height="600" fill="white"/>
<g transform="translate(400, 300)">"#,
        );

        if let Some(title) = &request.title {
            svg_content.push_str(&format!(
                r#"<text x="0" y="-250" text-anchor="middle" font-family="M PLUS 1p" font-size="20">{}</text>"#,
                title
            ));
        }

        let series = if request.series.is_empty() {
            let total: f64 = request.data.iter().sum();
            let colors = request.colors.clone().unwrap_or_default();
            vec![request
                .data
                .iter()
                .enumerate()
                .map(|(i, &value)| {
                    let color = colors.get(i).cloned();
                    (value / total * 100.0, color)
                })
                .collect::<Vec<_>>()]
        } else {
            request
                .series
                .iter()
                .map(|s| {
                    let total: f64 = s.data.iter().map(|d| d.value).sum();
                    s.data
                        .iter()
                        .map(|d| (d.value / total * 100.0, d.color.clone()))
                        .collect::<Vec<_>>()
                })
                .collect()
        };

        let radius = 180.0;
        let inner_radius = if self.is_donut { radius * 0.6 } else { 0.0 };
        let mut current_angle = -90.0; // Start from top

        for series_data in series {
            for (i, (percentage, color_opt)) in series_data.iter().enumerate() {
                let angle = 360.0 * percentage / 100.0;
                let end_angle = current_angle + angle;

                let color = color_opt.as_ref().map(String::as_str).unwrap_or_else(|| {
                    utils::get_default_colors()[i % utils::get_default_colors().len()]
                });

                // Calculate arc points
                let start_rad = current_angle * PI / 180.0;
                let end_rad = end_angle * PI / 180.0;

                let start_x = radius * start_rad.cos();
                let start_y = radius * start_rad.sin();
                let end_x = radius * end_rad.cos();
                let end_y = radius * end_rad.sin();

                let large_arc = if angle > 180.0 { 1 } else { 0 };

                if self.is_donut {
                    let inner_start_x = inner_radius * start_rad.cos();
                    let inner_start_y = inner_radius * start_rad.sin();
                    let inner_end_x = inner_radius * end_rad.cos();
                    let inner_end_y = inner_radius * end_rad.sin();

                    svg_content.push_str(&format!(
                        r#"<path d="M {:.1} {:.1} A {:.1} {:.1} 0 {} 1 {:.1} {:.1} L {:.1} {:.1} A {:.1} {:.1} 0 {} 0 {:.1} {:.1} Z" fill="{}" />"#,
                        start_x, start_y, radius, radius, large_arc, end_x, end_y,
                        inner_end_x, inner_end_y, inner_radius, inner_radius, large_arc, inner_start_x, inner_start_y,
                        color
                    ));
                } else {
                    svg_content.push_str(&format!(
                        r#"<path d="M {:.1} {:.1} A {:.1} {:.1} 0 {} 1 {:.1} {:.1} L 0 0 Z" fill="{}" />"#,
                        start_x, start_y, radius, radius, large_arc, end_x, end_y,
                        color
                    ));
                }

                // Add percentage label
                let label_angle = (current_angle + angle / 2.0) * PI / 180.0;
                let label_radius = radius * 0.75;
                let label_x = label_radius * label_angle.cos();
                let label_y = label_radius * label_angle.sin();

                svg_content.push_str(&format!(
                    r#"<text x="{:.1}" y="{:.1}" text-anchor="middle" font-family="M PLUS 1p" font-size="12">{:.1}%</text>"#,
                    label_x, label_y + 4.0, percentage
                ));

                current_angle = end_angle;
            }
        }

        svg_content.push_str("</g></svg>");
        svg_content
    }

    fn needs_axes(&self) -> bool {
        false
    }
}
