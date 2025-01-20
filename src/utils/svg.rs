use crate::models::Series;

pub fn create_svg_header(
    title: Option<&str>,
    x_label: Option<&str>,
    y_label: Option<&str>,
) -> String {
    let mut svg = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
<rect width="800" height="600" fill="white"/>
<g transform="translate(80, 50)">"#
    );

    if let Some(title) = title {
        svg.push_str(&format!(
            r#"<text x="320" y="30" text-anchor="middle" font-family="M PLUS 1p" font-size="20">{}</text>"#,
            title
        ));
    }

    if let Some(x_label) = x_label {
        svg.push_str(&format!(
            r#"<text x="320" y="520" text-anchor="middle" font-family="M PLUS 1p" font-size="14">{}</text>"#,
            x_label
        ));
    }

    if let Some(y_label) = y_label {
        svg.push_str(&format!(
            r#"<text x="-280" y="-50" text-anchor="middle" font-family="M PLUS 1p" font-size="14" transform="rotate(-90)">{}</text>"#,
            y_label
        ));
    }

    // Draw axes
    svg.push_str(
        r#"<line x1="0" y1="450" x2="640" y2="450" stroke="black" stroke-width="2"/>
<line x1="0" y1="50" x2="0" y2="450" stroke="black" stroke-width="2"/>"#,
    );

    svg
}

pub fn create_svg_footer() -> &'static str {
    "</g></svg>"
}

pub fn create_legend(series: &[Series], x: f64, y: f64) -> String {
    let mut legend = String::new();
    let mut y_offset = y;

    for series in series {
        if let Some(name) = &series.name {
            let default_color = "#000000".to_string();
            let color = series.color.as_ref().unwrap_or(&default_color);
            legend.push_str(&format!(
                r#"<rect x="{}" y="{}" width="20" height="20" fill="{}" />"#,
                x, y_offset, color
            ));
            legend.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="M PLUS 1p" font-size="12">{}</text>"#,
                x + 25.0,
                y_offset + 15.0,
                name
            ));
            y_offset += 25.0;
        }
    }

    legend
}

pub fn generate_y_axis_ticks(max_value: f64) -> String {
    let num_ticks = 5;
    let tick_step = max_value / num_ticks as f64;
    let mut ticks = String::new();

    for i in 0..=num_ticks {
        let y = 450.0 - ((450.0 - 50.0) * i as f64 / num_ticks as f64);
        let value = tick_step * i as f64;
        // Draw tick mark and label
        ticks.push_str(&format!(
            r#"<line x1="-5" y1="{y}" x2="0" y2="{y}" stroke="black" stroke-width="2"/>
            <text x="-10" y="{}" text-anchor="end" font-family="M PLUS 1p" font-size="12">{:.1}</text>"#,
            y + 4.0,
            value
        ));
        // Draw grid line
        ticks.push_str(&format!(
            r#"<line x1="0" y1="{0}" x2="640" y2="{0}" stroke="{1}" stroke-width="{2}" stroke-dasharray="{3}" />"#,
            y, "#CCCCCC", 1, 4
        ));
    }
    ticks
}

pub fn generate_x_axis_ticks(num_points: usize) -> String {
    let mut ticks = String::new();
    let segment_width = 640.0 / num_points as f64;

    for i in 0..num_points {
        let x = i as f64 * segment_width;
        ticks.push_str(&format!(
            r#"<line x1="{x}" y1="450" x2="{x}" y2="455" stroke="black" stroke-width="2"/>
            <text x="{x}" y="470" text-anchor="middle" font-family="M PLUS 1p" font-size="12">{}</text>"#,
            i + 1
        ));
    }
    ticks
}

pub fn generate_value_text(x: f64, y: f64, value: f64) -> String {
    format!(
        r#"<text x="{}" y="{}" text-anchor="middle" font-family="M PLUS 1p" font-size="12">{:.1}</text>"#,
        x,
        y - 5.0,
        value
    )
}
