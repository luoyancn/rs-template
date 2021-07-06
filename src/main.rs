#[macro_use]
extern crate slog_logger;

fn main() {
    slog_logger::setup_logger(false, "", 0, false, false);
    info!("Hello template");
    crit!("hello crit");
}
