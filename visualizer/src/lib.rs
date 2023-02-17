use chrono::Duration;
use plotters::prelude::*;

#[derive(serde::Deserialize, Copy, Clone)]
struct MachineMetricsData {
    timestamp: i64,
    cpu_usage: f32,
    memory_usage: f32,
}

pub fn visualization(name: String, file_path: &str) -> anyhow::Result<()> {
    println!("{:?}", file_path);
    let mut reader = csv::Reader::from_path(file_path).unwrap();
    let mut dataset: Vec<MachineMetricsData> = vec![];
    for record in reader.deserialize::<MachineMetricsData>() {
        let record = record?;
        dataset.push(record);
    }

    let start = dataset.first().unwrap().timestamp;
    let end = dataset.last().unwrap().timestamp;
    let distance = end - start;

    let png_path = format!("./temp/{}.png", name);
    let root = BitMapBackend::new(&png_path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(48, 48, 48, 48);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption(&name, ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(40)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..10f32, 0f32..100f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        .x_desc("Duration")
        .y_desc("Usage")
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(10)
        .y_labels(10)
        .x_label_style(("sans-serif", 24).into_font())
        .y_label_style(("sans-serif", 24).into_font())
        // We can also change the format of the label text
        .x_label_formatter(&|&x| {
            let during = Duration::seconds(((distance / 10) as f32 * x) as i64);
            return format!(
                "{:02}:{:02}:{:02}",
                during.num_hours(),
                during.num_minutes() % 60,
                during.num_seconds() % 60
            );
        })
        .y_label_formatter(&|y| format!("{:.1}%", y))
        .draw()?;

    let mut cpu_usage_set: Vec<(f32, f32)> = vec![];
    let mut memory_usage_set: Vec<(f32, f32)> = vec![];

    for data in dataset {
        let x = ((data.timestamp - start) as f32 / distance as f32) * 10.0;
        cpu_usage_set.push((x, data.cpu_usage));
        memory_usage_set.push((x, data.memory_usage));
    }

    // And we can draw something in the drawing area
    chart
        .draw_series(LineSeries::new(cpu_usage_set.clone(), RED))?
        .label("CPU Usage")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], RED.filled()));
    chart
        .draw_series(LineSeries::new(memory_usage_set.clone(), GREEN))?
        .label("MEM Usage")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.filled()));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        cpu_usage_set.clone(),
        2,
        &BLACK,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0, 0), s, st.filled()); // At this point, the new pixel coordinate is established
        },
    ))?;
    chart.draw_series(PointSeries::of_element(
        memory_usage_set.clone(),
        2,
        &BLACK,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0, 0), s, st.filled()); // At this point, the new pixel coordinate is established
        },
    ))?;

    root.present()?;
    Ok(())
}
