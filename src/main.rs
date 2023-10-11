extern "C" {
    fn print_num(num: i32);
}

fn main() {
    unsafe { print_num(10) };
}
