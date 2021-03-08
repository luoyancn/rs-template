#[macro_use]
extern crate slog_scope;
extern crate slog_logger;
extern crate slog_stdlog;

fn main() {
    let logger = slog_logger::initlogger(false, "", 0, false, false);
    let _guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    info!("Hello template");
    crit!("hello crit");
}
