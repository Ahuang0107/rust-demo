use std::ffi::OsStr;
use std::fs::DirEntry;

use chrono::Duration;
use plotters::prelude::*;

#[derive(serde::Deserialize, Copy, Clone)]
struct MachineMetricsData {
    timestamp: i64,
    cpu_usage: f32,
    memory_usage: f32,
}

fn main() -> anyhow::Result<()> {
    // 1. 首先先遍历一遍temp文件夹找出还没有生成相同文件名的png文件的csv文件
    let mut png_list: Vec<DirEntry> = vec![];
    let mut csv_list: Vec<DirEntry> = vec![];
    for dir_entry in std::fs::read_dir("./temp").unwrap() {
        let dir_entry = dir_entry?;
        if dir_entry.file_type().unwrap().is_file() {
            if dir_entry.path().extension() == Some(OsStr::new("png")) {
                png_list.push(dir_entry);
            } else if dir_entry.path().extension() == Some(OsStr::new("csv")) {
                csv_list.push(dir_entry);
            }
        }
    }
    let mut process_list: Vec<(String, &DirEntry)> = vec![];
    for csv in csv_list.as_slice() {
        let filename = csv.file_name();
        let captures = regex::Regex::new(r"(.*).csv")
            .unwrap()
            .captures(filename.to_str().unwrap())
            .unwrap();
        let csv_name = captures[1].parse::<String>().unwrap();
        let mut if_exist = false;
        for png in png_list.as_slice() {
            let filename = png.file_name();
            let captures = regex::Regex::new(r"(.*).png")
                .unwrap()
                .captures(filename.to_str().unwrap())
                .unwrap();
            let png_name = captures[1].parse::<String>().unwrap();
            if csv_name == png_name {
                if_exist = true;
            }
        }
        if !if_exist {
            process_list.push((csv_name, csv));
        }
    }
    // 2. 然后每个csv文件生成对应png文件
    for (name, file) in process_list {
        println!("{:?}", file.path());
        let mut reader = csv::Reader::from_path(file.path()).unwrap();
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
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(10)
            .y_labels(10)
            .x_label_style(("sans-serif", 24).into_font())
            .y_label_style(("sans-serif", 24).into_font())
            // We can also change the format of the label text
            .x_label_formatter(&|&x| {
                let during = Duration::seconds(((distance / 10) as f32 * x) as i64);
                // during.hou

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
        chart.draw_series(LineSeries::new(cpu_usage_set.clone(), &RED))?;
        chart.draw_series(LineSeries::new(memory_usage_set.clone(), &GREEN))?;

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
    }
    Ok(())
}
