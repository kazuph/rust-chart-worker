pub mod png;
pub mod svg;

pub fn get_default_colors() -> Vec<&'static str> {
    vec![
        "#0000FF", "#FFB3B3", "#B3E0FF", "#FFE6B3", "#B3FFB3", "#E6B3FF", "#FFD9B3",
    ]
}

pub fn format_number(num: f64) -> String {
    if num.fract() == 0.0 {
        format!("{:.0}", num)
    } else {
        format!("{:.1}", num)
    }
}
