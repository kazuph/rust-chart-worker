pub mod area;
pub mod bar;
pub mod line;
pub mod pie;
pub mod radar;
pub mod scatter;

use crate::models::{GraphRequest, GraphType, Series};

pub trait Chart {
    fn generate(&self, request: &GraphRequest) -> String;
}

pub fn create_chart(request: &GraphRequest) -> Box<dyn Chart> {
    match request.graph_type {
        GraphType::Bar => Box::new(bar::BarChart {}),
        GraphType::Line => Box::new(line::LineChart {}),
        GraphType::Scatter => Box::new(scatter::ScatterChart {}),
        GraphType::Pie => Box::new(pie::PieChart { is_donut: false }),
        GraphType::Donut => Box::new(pie::PieChart { is_donut: true }),
        GraphType::Area => Box::new(area::AreaChart {}),
        GraphType::Radar => Box::new(radar::RadarChart {}),
    }
}

pub fn get_max_value(series: &[Series]) -> f64 {
    series
        .iter()
        .flat_map(|s| s.data.iter())
        .map(|d| d.value)
        .fold(f64::NEG_INFINITY, f64::max)
}
