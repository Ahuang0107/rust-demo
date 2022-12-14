pub struct Reservation<'a> {
    id: &'a str,
    children: std::collections::HashMap<&'a str, Vec<Booking<'a>>>,
}

pub struct Booking<'a> {
    id: &'a str,
    start_time: f64,
    end_time: f64,
}
