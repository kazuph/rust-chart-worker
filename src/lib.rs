use plotters::prelude::*;
use plotters::series::LineSeries;
use serde::Deserialize;
use worker::*;

#[derive(Deserialize)]
struct GraphRequest {
    graph_type: String,
    data: Vec<f64>,
}

#[event(fetch)]
pub async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    if req.method() != Method::Post {
        return Response::error("Method not allowed", 405);
    }

    let graph_req: GraphRequest = match req.json().await {
        Ok(val) => val,
        Err(_) => return Response::error("Invalid JSON", 400),
    };

    if graph_req.data.is_empty() {
        return Response::error("Data array is empty", 400);
    }

    let svg = match create_chart(&graph_req) {
        Ok(s) => s,
        Err(e) => return Response::error(format!("Chart creation error: {}", e), 500),
    };

    let mut headers = Headers::new();
    headers.set("Content-Type", "image/svg+xml")?;

    Ok(Response::ok(svg)?.with_headers(headers))
}

fn create_chart(graph_req: &GraphRequest) -> core::result::Result<String, String> {
    let width = 800;
    let height = 600;

    let mut svg_data = String::new();
    {
        let root = SVGBackend::with_string(&mut svg_data, (width, height)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|e| format!("Failed to fill: {:?}", e))?;

        let max_val = graph_req.data.iter().cloned().fold(0.0, f64::max);
        let min_val = graph_req.data.iter().cloned().fold(max_val, f64::min);
        let padding = (max_val - min_val) * 0.1;

        let mut chart = ChartBuilder::on(&root)
            .margin(50)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                0..graph_req.data.len(),
                min_val - padding..max_val + padding,
            )
            .map_err(|e| format!("Failed to build chart: {:?}", e))?;

        chart
            .configure_mesh()
            .x_desc("Index")
            .y_desc("Value")
            .draw()
            .map_err(|e| format!("Failed to draw mesh: {:?}", e))?;

        match graph_req.graph_type.as_str() {
            "bar" => {
                chart
                    .draw_series(graph_req.data.iter().enumerate().map(|(idx, &val)| {
                        Rectangle::new([(idx, 0.0), (idx + 1, val)], RED.filled())
                    }))
                    .map_err(|e| format!("Failed to draw bar series: {:?}", e))?;
            }
            "scatter" => {
                chart
                    .draw_series(
                        graph_req
                            .data
                            .iter()
                            .enumerate()
                            .map(|(idx, &val)| Circle::new((idx, val), 5, BLUE.filled())),
                    )
                    .map_err(|e| format!("Failed to draw scatter series: {:?}", e))?;
            }
            _ => {
                chart
                    .draw_series(LineSeries::new(
                        graph_req
                            .data
                            .iter()
                            .enumerate()
                            .map(|(idx, &val)| (idx, val)),
                        &GREEN,
                    ))
                    .map_err(|e| format!("Failed to draw line series: {:?}", e))?;
            }
        }

        root.present()
            .map_err(|e| format!("Failed to present: {:?}", e))?;
    }

    Ok(svg_data)
}
