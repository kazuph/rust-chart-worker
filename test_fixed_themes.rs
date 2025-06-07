use rust_chart_worker::models::{GraphRequest, GraphType, Series, DataPoint};
use std::fs;

fn main() {
    let themes = vec!["light", "dark", "material", "minimal"];
    
    for theme in themes {
        let request = GraphRequest {
            graph_type: GraphType::Bar,
            title: Some(format!("修正済みバーチャート ({})", theme)),
            x_label: Some("X軸".to_string()),
            y_label: Some("Y軸".to_string()),
            data: vec![10.0, 20.0, 30.0, 25.0, 15.0],
            series: vec![
                Series {
                    name: Some("売上".to_string()),
                    data: vec![
                        DataPoint { value: 10.0, label: None, color: None },
                        DataPoint { value: 20.0, label: None, color: None },
                        DataPoint { value: 30.0, label: None, color: None },
                        DataPoint { value: 25.0, label: None, color: None },
                        DataPoint { value: 15.0, label: None, color: None },
                    ],
                    color: None,
                },
                Series {
                    name: Some("利益".to_string()),
                    data: vec![
                        DataPoint { value: 5.0, label: None, color: None },
                        DataPoint { value: 15.0, label: None, color: None },
                        DataPoint { value: 20.0, label: None, color: None },
                        DataPoint { value: 30.0, label: None, color: None },
                        DataPoint { value: 25.0, label: None, color: None },
                    ],
                    color: None,
                },
            ],
            theme: Some(theme.to_string()),
            bin_count: None,
            cell_size: None,
            show_values: None,
        };

        let chart = rust_chart_worker::charts::bar::BarChart {};
        let svg = rust_chart_worker::charts::Chart::generate(&chart, &request);
        
        let filename = format!("/data/data/com.termux/files/home/storage/shared/obsidian/attachment/fixed_bar_{}.svg", theme);
        fs::write(&filename, svg).expect("Failed to write SVG file");
        println!("Generated: {}", filename);
    }
    
    println!("テーマの修正完了！");
}