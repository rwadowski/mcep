pub mod json;

use crate::types::config::Logging;
use log::{error, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;

pub fn to_string<T: ToString>(t: T) -> String {
    t.to_string()
}

pub fn log_and_convert_to_string<T: ToString>(t: T) -> String {
    let s = t.to_string();
    error!("{}", s);
    s
}

pub fn configure_logger(logging: &Logging) {
    let level = if logging.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("kafka::consumer", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    let _ = log4rs::init_config(config).expect("logger should run");
}
