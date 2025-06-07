use super::Chart;
use crate::models::GraphRequest;
use crate::utils::{self, svg};
use crate::themes::ThemeManager;

pub struct LineChart {}

impl Chart for LineChart {
    fn generate(&self, request: &GraphRequest) -> String {
        // Get theme
        let theme = request.theme.as_ref()
            .map(|t| ThemeManager::from_name(t))
            .unwrap_or_else(|| ThemeManager::from_name("light"));

        let series = if request.series.is_empty() {
            vec![crate::models::Series {
                name: None,
                data: request.data.iter().map(|&value| crate::models::DataPoint {
                    value,
                    label: None,
                    color: None,
                }).collect(),
                color: Some(theme.get_color(0).clone()),
            }]
        } else {
            request.series.clone()
        };

        let max_value = series
            .iter()
            .flat_map(|s| s.data.iter().map(|d| d.value))
            .fold(f64::NEG_INFINITY, f64::max);

        let min_value = series
            .iter()
            .flat_map(|s| s.data.iter().map(|d| d.value))
            .fold(f64::INFINITY, f64::min);

        let value_range = max_value - min_value;
        let chart_height = 400.0;
        let chart_width = 640.0;
        let segment_width = chart_width / (series[0].data.len() as f64 - 1.0);

        let mut svg_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
<defs>
    <style>
        @font-face {{
            font-family: 'M PLUS 1p';
            src: url('data:font/truetype;base64,') format('truetype');
        }}
        text {{
            font-family: 'M PLUS 1p', 'Hiragino Sans', 'Yu Gothic', 'Meiryo', sans-serif;
        }}
    </style>
</defs>
<rect width="800" height="600" fill="{}"/>
<g transform="translate(80, 50)">"#,
            theme.background
        );

        // Add title if provided
        if let Some(title) = &request.title {
            svg_content.push_str(&format!(
                r#"<text fill="{}" x="320" y="30" text-anchor="middle" font-size="20">{}</text>"#,
                theme.text, title
            ));
        }

        // Add x-axis label if provided
        if let Some(x_label) = &request.x_label {
            svg_content.push_str(&format!(
                r#"<text fill="{}" x="320" y="520" text-anchor="middle" font-size="14">{}</text>"#,
                theme.text, x_label
            ));
        }

        // Add y-axis label if provided
        if let Some(y_label) = &request.y_label {
            svg_content.push_str(&format!(
                r#"<text fill="{}" x="-280" y="-50" text-anchor="middle" font-size="14" transform="rotate(-90)">{}</text>"#,
                theme.text, y_label
            ));
        }

        // Draw axes
        svg_content.push_str(&format!(
            r#"<line x1="0" y1="450" x2="640" y2="450" stroke="{}" stroke-width="2"/>
<line x1="0" y1="50" x2="0" y2="450" stroke="{}" stroke-width="2"/>"#,
            theme.axis, theme.axis
        ));

        // Draw y-axis ticks and grid
        for i in 0..=5 {
            let y = 450.0 - (i as f64 * 80.0);
            let value = min_value + (i as f64 * value_range / 5.0);
            svg_content.push_str(&format!(
                r#"<line x1="-5" y1="{}" x2="0" y2="{}" stroke="{}" stroke-width="2"/>
            <text fill="{}" x="-10" y="{}" text-anchor="end" font-size="12">{:.1}</text>"#,
                y, y, theme.axis, theme.text, y + 4.0, value
            ));
            if i > 0 {
                svg_content.push_str(&format!(
                    r#"<line x1="0" y1="{}" x2="640" y2="{}" stroke="{}" stroke-width="1" stroke-dasharray="4" />"#,
                    y, y, theme.grid
                ));
            }
        }

        // Draw x-axis ticks
        for i in 0..series[0].data.len() {
            let x = i as f64 * segment_width;
            svg_content.push_str(&format!(
                r#"<line x1="{}" y1="450" x2="{}" y2="455" stroke="{}" stroke-width="2"/>
            <text fill="{}" x="{}" y="470" text-anchor="middle" font-size="12">{}</text>"#,
                x, x, theme.axis, theme.text, x, i + 1
            ));
        }

        // Draw lines and points for each series
        for (series_idx, series_data) in series.iter().enumerate() {
            let color = match &series_data.color {
                Some(c) => c.clone(),
                None => theme.get_color(series_idx).clone(),
            };

            // Draw line path
            let mut path = String::new();
            for (i, point) in series_data.data.iter().enumerate() {
                let x = i as f64 * segment_width;
                let y = 450.0 - ((point.value - min_value) / value_range * chart_height);
                if i == 0 {
                    path.push_str(&format!("M {:.1} {:.1}", x, y));
                } else {
                    path.push_str(&format!(" L {:.1} {:.1}", x, y));
                }
            }
            svg_content.push_str(&format!(
                r#"<path d="{}" stroke="{}" stroke-width="2" fill="none" />"#,
                path, color
            ));

            // Draw points and value labels
            for (i, point) in series_data.data.iter().enumerate() {
                let x = i as f64 * segment_width;
                let y = 450.0 - ((point.value - min_value) / value_range * chart_height);
                svg_content.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="4" fill="{}" />"#,
                    x, y, color
                ));
                svg_content.push_str(&format!(
                    r#"<text fill="{}" x="{}" y="{}" text-anchor="middle" font-size="12">{:.1}</text>"#,
                    theme.text, x, y - 10.0, point.value
                ));
            }
        }

        // Add legend if there are multiple series
        if series.len() > 1 {
            let mut y_offset = 50.0;
            for (series_idx, series_data) in series.iter().enumerate() {
                if let Some(name) = &series_data.name {
                    let color = match &series_data.color {
                        Some(c) => c.clone(),
                        None => theme.get_color(series_idx).clone(),
                    };
                    svg_content.push_str(&format!(
                        r#"<rect x="520" y="{}" width="20" height="20" fill="{}" />"#,
                        y_offset, color
                    ));
                    svg_content.push_str(&format!(
                        r#"<text fill="{}" x="545" y="{}" font-size="12">{}</text>"#,
                        theme.text, y_offset + 15.0, name
                    ));
                    y_offset += 25.0;
                }
            }
        }

        svg_content.push_str("</g></svg>");
        svg_content
    }
}
