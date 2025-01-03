use image::Rgba;
use imageproc::contours::{find_contours, BorderType};
use imageproc::point::Point;
use std::collections::BTreeSet;

fn main() {
    extract_contours("trigger_area_test_big");
    extract_contours("trigger_area_test_middle");
    extract_contours("trigger_area_test_small");
}

/// TODO 正在实现提取和简化形状的逻辑
fn extract_contours(filename: &str) {
    let rgba_file = format!("assets/{filename}.png");
    let luma_file = format!("assets/[LUMA]{filename}.png");
    let origin_outline_file = format!("assets/[Origin Outline]{filename}.png");
    let simple_outline_1_file = format!("assets/[Simple Outline 1]{filename}.png");
    let simple_outline_2_file = format!("assets/[Simple Outline 2]{filename}.png");

    let img = image::open(rgba_file)
        .expect("Failed to open image")
        .to_luma8();

    img.save(luma_file).unwrap();

    let mut origin_outline = image::RgbaImage::new(img.width(), img.height());
    let mut simple_outline_1 = image::RgbaImage::new(img.width(), img.height());
    let mut simple_outline_2 = image::RgbaImage::new(img.width(), img.height());
    let contours = find_contours::<u32>(&img);

    for contour in contours {
        println!("Contour: {:?} {:?}", contour.border_type, contour.parent);
        let color: Rgba<u8> = match contour.border_type {
            BorderType::Outer => Rgba::from([0, 0, 0, 255]),
            BorderType::Hole => Rgba::from([255, 255, 255, 255]),
        };

        let points = contour.points;
        for point in points.iter() {
            origin_outline.put_pixel(point.x, point.y, color);
        }

        let simplified_1 = simplify(&points);

        for i in simplified_1.iter() {
            let point = &points[*i];
            simple_outline_1.put_pixel(point.x, point.y, color);
        }

        let skipped = step2_skip(&points);

        let simplified_2 = simplified_1.difference(&skipped);

        for i in simplified_2 {
            let point = &points[*i];
            simple_outline_2.put_pixel(point.x, point.y, color);
        }
    }

    origin_outline.save(origin_outline_file).unwrap();
    simple_outline_1.save(simple_outline_1_file).unwrap();
    simple_outline_2.save(simple_outline_2_file).unwrap();
}

#[derive(Debug)]
struct Slope(i32, i32);

impl PartialEq for Slope {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == 0 && self.1 == 0 {
            return false;
        }
        if other.0 == 0 && other.1 == 0 {
            return false;
        }
        self.0 * other.1 == self.1 * other.0
    }
}

fn simplify(points: &Vec<Point<u32>>) -> BTreeSet<usize> {
    let mut simplified = BTreeSet::new();
    simplified.insert(0);
    let mut previous_slope = Slope(
        points[1].x as i32 - points[0].x as i32,
        points[1].y as i32 - points[0].y as i32,
    );
    for i in 1..points.len() {
        let next_index = if i == (points.len() - 1) { 0 } else { i + 1 };
        let slope = Slope(
            points[next_index].x as i32 - points[i].x as i32,
            points[next_index].y as i32 - points[i].y as i32,
        );

        // 只有是第一个方向向量，或者方向发生了变化时记录末尾点，更新方向向量
        // 否则直接跳过
        if slope == previous_slope {
            // 相等则表示 i 指向 i+1 的方向和 i-1 指向 i 的方向是相同的
            // 需要忽略跳过 i 这个 点
        } else {
            // 结束当前段，记录末尾点
            simplified.insert(i);
        }

        previous_slope = slope;
    }

    simplified
}

fn step2_skip(points: &Vec<Point<u32>>) -> BTreeSet<usize> {
    let mut skip = BTreeSet::new();
    let mut previous_slope_1 = Slope(
        points[1].x as i32 - points[0].x as i32,
        points[1].y as i32 - points[0].y as i32,
    );
    let mut previous_slope_2 = Slope(
        points[2].x as i32 - points[1].x as i32,
        points[2].y as i32 - points[1].y as i32,
    );
    for i in 2..points.len() {
        let next_index = (i + 1) % points.len();
        let next_next_index = (i + 2) % points.len();
        let slope_1 = Slope(
            points[next_index].x as i32 - points[i].x as i32,
            points[next_index].y as i32 - points[i].y as i32,
        );
        let slope_2 = Slope(
            points[next_next_index].x as i32 - points[next_index].x as i32,
            points[next_index].y as i32 - points[next_index].y as i32,
        );

        if slope_1 == previous_slope_1 && slope_2 == previous_slope_2 {
            skip.insert(i - 1);
            skip.insert(i);
            skip.insert(i + 1);
        }

        previous_slope_1 = previous_slope_2;
        previous_slope_2 = slope_1;
    }

    skip
}
