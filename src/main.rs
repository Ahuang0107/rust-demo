mod app;
mod common;
mod event;
mod express;
mod monitor;
mod resource;
mod slam_club;
mod statistics;
mod talent;

fn main() {
    eframe::run_native(
        "Steam Tracker",
        eframe::NativeOptions {
            vsync: true,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
    .unwrap();
}
