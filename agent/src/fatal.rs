#[macro_export]
macro_rules! fatal {
    () => {
        log::error!("config panic");
        panic!()
    };
    ($msg:expr) => {
        log::error!($msg);
        panic!()
    };
    ($msg:expr,) => {
        log::error!($msg);
        panic!()
    };
    ($fmt:expr, $($arg:tt)+) => {
        let msg = format!($fmt, $($arg)+);
        log::error!("{}", &msg);
        panic!()
    };
}
