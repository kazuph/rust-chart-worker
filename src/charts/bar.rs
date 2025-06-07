use crate::models::{GraphRequest, Series};
use crate::utils::{self, svg};
use crate::themes::ThemeManager;

pub struct BarChart {}

impl super::Chart for BarChart {
    fn generate(&self, request: &GraphRequest) -> String {
        // Get theme
        let theme = request.theme.as_ref()
            .map(|t| ThemeManager::from_name(t))
            .unwrap_or_else(|| ThemeManager::from_name("light"));
            
        let default_series = if request.series.is_empty() {
            let default_color = theme.get_color(0).clone();
            let series_data = request
                .data
                .iter()
                .map(|&value| crate::models::DataPoint {
                    value,
                    label: None,
                    color: None,
                })
                .collect();
            vec![Series {
                name: None,
                data: series_data,
                color: Some(default_color),
            }]
        } else {
            request.series.clone()
        };

        let max_value = default_series
            .iter()
            .flat_map(|s| s.data.iter().map(|d| d.value))
            .fold(f64::NEG_INFINITY, f64::max);

        let segment_width = 700.0 / (default_series[0].data.len() as f64);
        let bar_width = segment_width * 0.8; // 80% of segment width for the bar

        let mut svg_content = format!(
            r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
            <rect width="100%" height="100%" fill="{}"/>"#,
            theme.background
        );

        // Add title if provided
        if let Some(title) = &request.title {
            svg_content.push_str(&format!(
                r#"<text fill="{}" x="400" y="50" text-anchor="middle" font-family="M PLUS 1p" font-size="20">{}</text>"#,
                theme.text, title
            ));
        }

        // Add x-axis label if provided
        if let Some(x_label) = &request.x_label {
            svg_content.push_str(&format!(
                r#"<text fill="{}" x="400" y="580" text-anchor="middle" font-family="M PLUS 1p" font-size="14">{}</text>"#,
                theme.text, x_label
            ));
        }

        // Add y-axis label if provided
        if let Some(y_label) = &request.y_label {
            svg_content.push_str(&format!(
                r#"<text fill="{}" x="30" y="300" text-anchor="middle" font-family="M PLUS 1p" font-size="14" transform="rotate(-90, 30, 300)">{}</text>"#,
                theme.text, y_label
            ));
        }

        // Draw axes
        svg_content.push_str(&format!(
            r#"<line x1="50" y1="500" x2="750" y2="500" style="stroke:{};stroke-width:2"/>
            <line x1="50" y1="100" x2="50" y2="500" style="stroke:{};stroke-width:2"/>"#,
            theme.axis, theme.axis
        ));

        // Draw y-axis ticks and values
        for i in 0..=5 {
            let y = 500.0 - (i as f64 * 80.0);
            let value = (i as f64 * max_value / 5.0).round();
            svg_content.push_str(&format!(
                r#"<line x1="45" y1="{}" x2="50" y2="{}" style="stroke:{};stroke-width:1"/>
                <text fill="{}" x="40" y="{}" text-anchor="end" font-family="M PLUS 1p" font-size="12">{}</text>"#,
                y, y, theme.axis, theme.text, y + 4.0, value
            ));
        }

        // Draw x-axis ticks and values
        for i in 0..default_series[0].data.len() {
            let x = 50.0 + (i as f64 * segment_width) + (segment_width / 2.0);
            svg_content.push_str(&format!(
                r#"<line x1="{}" y1="500" x2="{}" y2="505" style="stroke:{};stroke-width:1"/>
                <text fill="{}" x="{}" y="520" text-anchor="middle" font-family="M PLUS 1p" font-size="12">{}</text>"#,
                x, x, theme.axis, theme.text, x, i + 1
            ));
        }

        // Draw bars
        for (series_idx, series_item) in default_series.iter().enumerate() {
            let series_offset = (series_idx as f64 - (default_series.len() as f64 - 1.0) / 2.0)
                * (bar_width / default_series.len() as f64);

            for (i, point) in series_item.data.iter().enumerate() {
                let x = 50.0
                    + (i as f64 * segment_width)
                    + (segment_width - bar_width) / 2.0
                    + series_offset;
                let height = (point.value / max_value) * 400.0;
                let y = 500.0 - height;
                let color = match &point.color {
                    Some(c) => c.clone(),
                    None => match &series_item.color {
                        Some(c) => c.clone(),
                        None => theme.get_color(series_idx).clone(),
                    },
                };

                svg_content.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
                    x,
                    y,
                    bar_width / default_series.len() as f64,
                    height,
                    color
                ));

                svg_content.push_str(&format!(
                    r#"<text fill="{}" x="{}" y="{}" text-anchor="middle" font-size="12">{}</text>"#,
                    theme.text,
                    x + (bar_width / default_series.len() as f64) / 2.0,
                    y - 10.0,
                    point.value
                ));
            }
        }

        // Add legend if there are multiple series
        if default_series.len() > 1 {
            let mut y_offset = 50.0;
            for (series_idx, series) in default_series.iter().enumerate() {
                if let Some(name) = &series.name {
                    let color = match &series.color {
                        Some(c) => c.clone(),
                        None => theme.get_color(series_idx).clone(),
                    };
                    svg_content.push_str(&format!(
                        r#"<rect x="520" y="{}" width="20" height="20" fill="{}" />"#,
                        y_offset, color
                    ));
                    svg_content.push_str(&format!(
                        r#"<text fill="{}" x="545" y="{}" font-family="M PLUS 1p" font-size="12">{}</text>"#,
                        theme.text, y_offset + 15.0, name
                    ));
                    y_offset += 25.0;
                }
            }
        }

        svg_content.push_str("</svg>");
        svg_content
    }
}
