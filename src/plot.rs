use plotters::prelude::*;

const IMAGE_WIDTH: u32 = 640;
const IMAGE_HEIGHT: u32 = 480;

const LINE_COLOR: RGBColor = RGBColor(220, 50, 90);
const POINT_COLOR: RGBColor = RGBColor(250, 192, 61);

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
    let get_min_max = |values: &[f64]| {
        let mut min = values[0];
        let mut max = values[0];
        for &value in values.iter() {
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
        }
        (min, max)
    };

    let root = SVGBackend::new(file_name, (IMAGE_WIDTH, IMAGE_HEIGHT)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let (min_y, max_y) = get_min_max(y_values);

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(x_values[0]..x_values[x_values.len() - 1], min_y..max_y)
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

    chart
        .draw_series(
            x_values
                .iter()
                .zip(y_values.iter())
                .map(|(x, y)| Circle::new((*x, *y), 1, POINT_COLOR.filled())),
        )
        .unwrap();
}
