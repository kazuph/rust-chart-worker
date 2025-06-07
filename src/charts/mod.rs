pub mod area;
pub mod bar;
pub mod line;
pub mod pie;
pub mod radar;
pub mod scatter;
pub mod histogram;
pub mod heatmap;
pub mod candlestick;
pub mod gauge;
pub mod bubble;
pub mod stacked_bar;
pub mod multi_line;

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
        GraphType::Histogram => {
            let mut chart = histogram::HistogramChart::default();
            if let Some(bins) = request.bins {
                chart.bins = bins;
            }
            if let Some(show_density) = request.show_density {
                chart.show_density = show_density;
            }
            Box::new(chart)
        },
        GraphType::Heatmap => {
            let mut chart = heatmap::HeatmapChart::default();
            if let Some(cell_size) = request.cell_size {
                chart.cell_size = cell_size;
            }
            if let Some(show_values) = request.show_values {
                chart.show_values = show_values;
            }
            Box::new(chart)
        },
        GraphType::Candlestick => Box::new(candlestick::CandlestickChart {}),
        GraphType::Gauge => {
            let mut chart = gauge::GaugeChart::default();
            // パラメータから設定を取得（オプション）
            Box::new(chart)
        },
        GraphType::Bubble => {
            let mut chart = bubble::BubbleChart::default();
            if let Some(show_values) = request.show_values {
                chart.show_labels = show_values;
            }
            Box::new(chart)
        },
        GraphType::StackedBar => {
            let mut chart = stacked_bar::StackedBarChart::default();
            if let Some(show_values) = request.show_values {
                chart.show_values = show_values;
            }
            Box::new(chart)
        },
        GraphType::MultiLine => {
            Box::new(multi_line::MultiLineChart::default())
        },
    }
}

pub fn get_max_value(series: &[Series]) -> f64 {
    series
        .iter()
        .flat_map(|s| s.data.iter())
        .map(|d| d.value)
        .fold(f64::NEG_INFINITY, f64::max)
}
