use plotters::prelude::*;

const IMAGE_WIDTH: u32 = 640;
const IMAGE_HEIGHT: u32 = 480;

const LINE_COLOR: RGBColor = RGBColor(220, 50, 90);

pub struct PlotInfo<'a, 'b> {
    x_values: &'a [f64],
    y_values: &'a [f64],
    x_label: &'b str,
    y_label: &'b str,
    caption: &'b str,
}

impl<'a, 'b> PlotInfo<'a, 'b> {
    pub fn new(
        x_values: &'a [f64],
        y_values: &'a [f64],
        x_label: &'b str,
        y_label: &'b str,
        caption: &'b str,
    ) -> Self {
        Self {
            x_values,
            y_values,
            x_label,
            y_label,
            caption,
        }
    }
}

pub fn plot(plot_info: PlotInfo, file_name: &str) {
    let PlotInfo {
        x_values,
        y_values,
        x_label,
        y_label,
        caption,
    } = plot_info;
    let root = BitMapBackend::new(file_name, (IMAGE_WIDTH, IMAGE_HEIGHT)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_values[0]..x_values[x_values.len() - 1], 0f64..10f64)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            x_values.iter().zip(y_values.iter()).map(|(x, y)| (*x, *y)),
            &LINE_COLOR,
        ))
        .unwrap();
}
