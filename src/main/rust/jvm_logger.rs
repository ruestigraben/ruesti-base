use crate::polyglot::*;
use log;
use std::fmt::format;

struct JvmLogger;

static LOGGER: JvmLogger = JvmLogger;

impl log::Log for JvmLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let execute = as_executable(import("__base_logging\0"));
        let record_type = java_type("dev.ruestigraben.base.logging.Record\0");

        let level = match record.metadata().level() {
            log::Level::Trace => "TRACE",
            log::Level::Debug => "DEBUG",
            log::Level::Error => "ERROR",
            log::Level::Info => "INFO",
            log::Level::Warn => "WARN"
        };

        let args = from_string(format(*record.args()).as_str());

        let file = record.file().unwrap_or("");
        let line = record.line().map(|l| l as i64).unwrap_or(-1);

        let java_record = unsafe {
            polyglot_new_instance(
                record_type,
                from_string(level),
                from_string(record.metadata().target()),
                args,
                from_string(file),
                line
            )
        };

        let _ = invoke(execute, java_record);
    }

    fn flush(&self) {
    }
}

#[no_mangle]
pub fn __base_init_logger() -> () {
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace));
}