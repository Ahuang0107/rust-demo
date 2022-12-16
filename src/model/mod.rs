#[allow(dead_code)]
pub struct Reservation<'a> {
    id: &'a str,
    children: std::collections::HashMap<&'a str, Vec<Booking<'a>>>,
}

#[allow(dead_code)]
pub struct Booking<'a> {
    id: &'a str,
    start_time: f64,
    end_time: f64,
}
