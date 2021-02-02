extern crate slog_scope;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

extern crate slog_logger;

fn main() {
    let logger = slog_logger::initlogger(false, "", 0);
    let _guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    info!("Hello template");
}
