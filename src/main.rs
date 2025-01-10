use image::Rgba;
use imageproc::contours::{find_contours, BorderType};
use imageproc::point::Point;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

fn main() {
    extract_contours("trigger_area_test_big");
    extract_contours("trigger_area_test_middle");
    extract_contours("trigger_area_test_small");
}

/// TODO 正在实现提取和简化形状的逻辑
///  目前这里的主要问题是，分步骤简化的话，会出现，第一步简化后失真了，导致第二步简化是忽略了不应该忽略的点。比如 [Simple Outline 2]trigger_area_test_big 的右上角凹进去的地方
///  感觉需要调整为，先找出所有关键转折处的点（但是这里找出来的点可能有冗余），然后在找出step为2的连续直线的中间点，step为3的连续直线的中间内容，这部分点从之前的关键转折点中剔除掉
/// TODO 应该专注要找直线，不应该局部判断，而是需要全局判断，就是找到一条完整的直线，然后记录起始点和终点，忽略中间的所有点
///  然后再找间隔为 2 形成的直线，也是保留起始点和终点，忽略中间所有的点，并且优先级比上面的高，如果忽略的点在上面的起始点内，
fn extract_contours(filename: &str) {
    let rgba_file = format!("assets/[White]{filename}.png");
    let luma_file = format!("assets/[LUMA]{filename}.png");
    let outline_file = format!("assets/[Outline]{filename}.png");
    let polygon_file = format!("assets/[Polygon]{filename}.png");

    let img = image::open(rgba_file).expect("Failed to open image");
    let luma8_image = img.to_luma8();

    luma8_image.save(luma_file).unwrap();

    let mut outline = img.to_rgba8();
    let mut polygon = img.to_rgba8();
    // 1. 先提取出轮廓
    let contours = find_contours::<u32>(&luma8_image);

    for contour in contours {
        println!("Contour: {:?} {:?}", contour.border_type, contour.parent);
        let color: Rgba<u8> = match contour.border_type {
            BorderType::Outer => Rgba::from([0, 0, 0, 255]),
            BorderType::Hole => Rgba::from([255, 255, 255, 255]),
        };

        let points = contour.points;

        for point in points.iter() {
            outline.put_pixel(point.x, point.y, color);
        }

        for line in find_lines(&points) {
            for i in *line.start()..=*line.end() {
                let point = points[i];
                polygon.put_pixel(point.x, point.y, color);
            }
        }
    }

    outline.save(outline_file).unwrap();
    polygon.save(polygon_file).unwrap();
}

#[derive(Debug, Default, Copy, Clone)]
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

fn find_lines(points: &Vec<Point<u32>>) -> Vec<RangeInclusive<usize>> {
    let mut lines = Vec::new();
    let mut previous_slope = Slope(
        points[1].x as i32 - points[0].x as i32,
        points[1].y as i32 - points[0].y as i32,
    );
    let mut cur_line: Option<RangeInclusive<usize>> = None;
    for i in 1..points.len() {
        let cur_index = i % points.len();
        let next_index = (i + 1) % points.len();
        let slope = Slope(
            points[next_index].x as i32 - points[cur_index].x as i32,
            points[next_index].y as i32 - points[cur_index].y as i32,
        );

        if slope == previous_slope {
            if let Some(cur_line) = &mut cur_line {
                *cur_line = *cur_line.start()..=next_index;
            } else {
                cur_line = Some(cur_index - 1..=next_index);
            }
        } else {
            previous_slope = slope;
            if let Some(cur_line) = cur_line.take() {
                lines.push(cur_line);
            }
        }
    }
    lines
}

fn find_skip_points_step_1(points: &Vec<Point<u32>>) -> BTreeSet<usize> {
    let mut skip = BTreeSet::new();
    let mut previous_slope = Slope(
        points[1].x as i32 - points[0].x as i32,
        points[1].y as i32 - points[0].y as i32,
    );
    for i in 1..points.len() {
        let cur_index = i % points.len();
        let next_index = (i + 1) % points.len();
        let slope = Slope(
            points[next_index].x as i32 - points[cur_index].x as i32,
            points[next_index].y as i32 - points[cur_index].y as i32,
        );

        if slope == previous_slope {
            skip.insert(i);
        }

        previous_slope = slope;
    }

    skip
}

/// 进一步把连续两个
fn find_skip_points_step_2(points: &Vec<Point<u32>>) -> BTreeSet<usize> {
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
        let cur_index = i % points.len();
        let next_index = (i + 1) % points.len();
        let next_next_index = (i + 2) % points.len();
        let slope_1 = Slope(
            points[next_index].x as i32 - points[cur_index].x as i32,
            points[next_index].y as i32 - points[cur_index].y as i32,
        );
        let slope_2 = Slope(
            points[next_next_index].x as i32 - points[next_index].x as i32,
            points[next_next_index].y as i32 - points[next_index].y as i32,
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
