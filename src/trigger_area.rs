use geo::{Coord, LineString, Polygon, Simplify};
use image::{GenericImage, Rgba};
use imageproc::contours::{find_contours, BorderType};
use imageproc::point::Point;
use std::collections::BTreeSet;

pub struct TriggerArea {
    pub polygon: Polygon<f32>,
    pub offset: [f32; 2],
}

impl TriggerArea {
    pub fn new(filename: &str) -> Self {
        let rgba_file = format!("assets/1[White]{filename}.png");
        let luma8_file = format!("assets/2[LUMA]{filename}.png");
        let outline_file = format!("assets/3[Outline]{filename}.png");
        let outline_2_file = format!("assets/4[Outline 2]{filename}.png");
        let outline_3_file = format!("assets/5[Outline 3]{filename}.png");
        let outline_4_file = format!("assets/6[Outline 4]{filename}.png");
        let polygon_file = format!("assets/7[Polygon]{filename}.png");
        let polygon_2_file = format!("assets/8[Polygon 2]{filename}.png");

        let img = image::open(rgba_file).expect("Failed to open image");
        let mut outline_image = img.clone();
        let mut outline_2_image = img.clone();
        let mut outline_3_image = img.clone();
        let mut outline_4_image = img.clone();
        let mut polygon_image = img.clone();
        let mut polygon_2_image = img.clone();
        let luma8_image = img.to_luma8();
        let _ = luma8_image.save(&luma8_file);

        let contours = find_contours::<u32>(&luma8_image);
        assert_eq!(contours.len(), 1);

        let mut exterior = None;
        let mut interiors = Vec::new();
        for contour in contours {
            let points = contour.points;

            for point in points.iter() {
                outline_image.put_pixel(point.x, point.y, Rgba([0, 0, 0, 255]));
            }

            let _ = outline_image.save(&outline_file);

            // let mut new_points = Vec::new();
            // let skips = find_skip_points_step_3(&points);
            // for (index, point) in points.iter().enumerate() {
            //     if skips.contains(&index) {
            //         continue;
            //     }
            //     new_points.push(*point);
            // }

            let mut new_points_2 = Vec::new();
            let skips = find_skip_points_step_2(&points);
            for (index, point) in points.iter().enumerate() {
                if skips.contains(&index) {
                    continue;
                }
                outline_2_image.put_pixel(point.x, point.y, Rgba([0, 0, 0, 255]));
                new_points_2.push(*point);
            }
            let _ = outline_2_image.save(&outline_2_file);

            let mut new_points_3 = Vec::new();
            let skips = find_skip_points_step_1(&new_points_2);
            for (index, point) in new_points_2.iter().enumerate() {
                if skips.contains(&index) {
                    continue;
                }
                outline_3_image.put_pixel(point.x, point.y, Rgba([0, 0, 0, 255]));
                new_points_3.push(*point);
            }
            let _ = outline_3_image.save(&outline_3_file);

            let mut coords = Vec::new();
            for point in new_points_3 {
                outline_4_image.put_pixel(point.x, point.y, Rgba([0, 0, 0, 255]));
                coords.push(Coord::from([point.x as f32, point.y as f32]));
            }
            let _ = outline_4_image.save(&outline_4_file);

            match contour.border_type {
                BorderType::Outer => {
                    exterior = Some(LineString::new(coords));
                }
                BorderType::Hole => {
                    interiors.push(LineString::new(coords));
                }
            };
        }

        let polygon = Polygon::new(exterior.unwrap(), interiors);

        for point in polygon.exterior() {
            let x = point.x as u32;
            let y = point.y as u32;
            polygon_image.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }

        let _ = polygon_image.save(polygon_file);

        let polygon2 = polygon.simplify(&1.0);

        for point in polygon2.exterior() {
            let x = point.x as u32;
            let y = point.y as u32;
            polygon_2_image.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }

        let _ = polygon_2_image.save(polygon_2_file);

        Self {
            polygon,
            offset: [0.0, 0.0],
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Slope(i32, i32);

impl Slope {
    pub fn if_vh(&self) -> bool {
        self.0 == 0 || self.1 == 0
    }
}

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

fn find_skip_points_step_2(points: &Vec<Point<u32>>) -> BTreeSet<usize> {
    let mut skip = BTreeSet::new();
    for i in 0..=points.len() {
        let cur_index = i % points.len();
        let next_index = (i + 1) % points.len();
        let next_next_index = (i + 2) % points.len();
        let next_next_next_index = (i + 3) % points.len();
        let next_next_next_next_index = (i + 4) % points.len();
        let log = (points[cur_index].x == 156 && points[cur_index].y == 52)
            || (points[cur_index].x == 157 && points[cur_index].y == 53)
            || (points[cur_index].x == 157 && points[cur_index].y == 54)
            || (points[cur_index].x == 158 && points[cur_index].y == 55);

        let slope_1 = Slope(
            points[next_index].x as i32 - points[cur_index].x as i32,
            points[next_index].y as i32 - points[cur_index].y as i32,
        );
        let slope_2 = Slope(
            points[next_next_index].x as i32 - points[next_index].x as i32,
            points[next_next_index].y as i32 - points[next_index].y as i32,
        );
        let slope_3 = Slope(
            points[next_next_next_index].x as i32 - points[next_next_index].x as i32,
            points[next_next_next_index].y as i32 - points[next_next_index].y as i32,
        );
        let slope_4 = Slope(
            points[next_next_next_next_index].x as i32 - points[next_next_next_index].x as i32,
            points[next_next_next_next_index].y as i32 - points[next_next_next_index].y as i32,
        );

        if log {
            println!("{slope_1:?} {slope_2:?} {slope_3:?} {slope_4:?}")
        }

        if slope_1.if_vh() && slope_3.if_vh() && !slope_2.if_vh() && !slope_4.if_vh() {
            skip.insert(next_index);
            skip.insert(next_next_index);
            skip.insert(next_next_next_index);
        }
    }

    skip
}

fn find_skip_points_step_3(points: &Vec<Point<u32>>) -> BTreeSet<usize> {
    let mut skip = BTreeSet::new();
    let mut previous_slope_1 = Slope(
        points[1].x as i32 - points[0].x as i32,
        points[1].y as i32 - points[0].y as i32,
    );
    let mut previous_slope_2 = Slope(
        points[2].x as i32 - points[1].x as i32,
        points[2].y as i32 - points[1].y as i32,
    );
    let mut previous_slope_3 = Slope(
        points[3].x as i32 - points[2].x as i32,
        points[3].y as i32 - points[2].y as i32,
    );
    for i in 3..points.len() {
        let cur_index = i % points.len();
        let next_index = (i + 1) % points.len();
        let next_next_index = (i + 2) % points.len();
        let next_next_next_index = (i + 3) % points.len();
        let slope_1 = Slope(
            points[next_index].x as i32 - points[cur_index].x as i32,
            points[next_index].y as i32 - points[cur_index].y as i32,
        );
        let slope_2 = Slope(
            points[next_next_index].x as i32 - points[next_index].x as i32,
            points[next_next_index].y as i32 - points[next_index].y as i32,
        );
        let slope_3 = Slope(
            points[next_next_next_index].x as i32 - points[next_next_index].x as i32,
            points[next_next_next_index].y as i32 - points[next_next_index].y as i32,
        );

        if slope_1 == previous_slope_1 && slope_2 == previous_slope_2 && slope_3 == previous_slope_3
        {
            skip.insert(i - 2);
            skip.insert(i - 1);
            skip.insert(i);
            skip.insert(i + 1);
            skip.insert(i + 2);
        }

        previous_slope_1 = previous_slope_2;
        previous_slope_2 = previous_slope_3;
        previous_slope_3 = slope_1;
    }

    skip
}
