use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Handle,
};

pub fn configure(level: log::LevelFilter) -> Result<Handle, anyhow::Error> {
    // file path to log subfolder with log + timestamp.log
    let file_path = format!(
        "logs/{}.log",
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
    );

    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    let encoder = PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} | {h({l:5.5})} | {m}\n");


    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(encoder.clone()))
        .target(Target::Stderr)
        .build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        .encoder(Box::new(encoder))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let handle = log4rs::init_config(config)?;

    Ok(handle)
}
