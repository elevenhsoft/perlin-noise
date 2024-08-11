use plotters::prelude::*;
// use rand::{thread_rng, Rng};

fn main() {
    let mut perlin = perlin_noise::Perlin::new(0);

    let mut series: Vec<(f64, f64)> = Vec::new();

    for n in 0..10_000 {
        let mut x = n as f64 * 0.01;
        let mut y = n as f64 * 0.01;
        let z = n as f64 * 0.5;

        x = perlin.octave(x, y, z, 10, 1.0);
        y = perlin.octave(x, y, z, 10, 1.0);

        series.push((x, y));
    }

    let root = BitMapBackend::new("plot.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let areas = root.split_by_breakpoints([944], [80]);

    let mut x_hist_ctx = ChartBuilder::on(&areas[0])
        .y_label_area_size(40)
        .build_cartesian_2d((0.0..1.0).step(0.01).use_round().into_segmented(), 0..250)
        .unwrap();
    let mut y_hist_ctx = ChartBuilder::on(&areas[3])
        .x_label_area_size(40)
        .build_cartesian_2d(0..250, (0.0..1.0).step(0.01).use_round())
        .unwrap();
    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)
        .unwrap();
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();
    scatter_ctx
        .draw_series(
            series
                .iter()
                .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
        )
        .unwrap();
    let x_hist = Histogram::vertical(&x_hist_ctx)
        .style(GREEN.filled())
        .margin(0)
        .data(series.iter().map(|(x, _)| (*x, 1)));
    let y_hist = Histogram::horizontal(&y_hist_ctx)
        .style(GREEN.filled())
        .margin(0)
        .data(series.iter().map(|(_, y)| (*y, 1)));
    x_hist_ctx.draw_series(x_hist).unwrap();
    y_hist_ctx.draw_series(y_hist).unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
}
