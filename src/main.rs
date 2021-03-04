extern crate slog_scope;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

extern crate slog_logger;

use slog_stdlog::crit;

fn main() {
    let logger = slog_logger::initlogger(false, "", 0, false, false);
    let _guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    info!("Hello template");
    crit!("hello crit");
}
