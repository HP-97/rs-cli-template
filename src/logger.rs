use tracing::{debug, info, level_filters::LevelFilter, warn};
use tracing_subscriber::{filter, fmt, prelude::*, reload, Registry};

/// Sets up the global logger for the whole application.
///
/// Date is captured in UTC time.
pub fn setup_logging<S>(_log_level: usize)
where
    tracing_subscriber::reload::Layer<LevelFilter, S>: __tracing_subscriber_Layer<Registry>,
{
    // Documentation on implemention a Layer wrapped in an Option
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/
    let (level_filter, reload_handle) = reload::Layer::<LevelFilter, S>::new(LevelFilter::WARN);
    tracing_subscriber::registry()
        .with(level_filter)
        .with(fmt::Layer::default().with_writer(std::io::stderr))
        .init();

    // let log_level = Level::from_str(&log_level.to_string()).unwrap();
    // let filter = filter::LevelFilter::from_level(log_level);
    // let (filter, reload_handle) = reload::Layer::<LevelFilter, S>::new(filter);
    // tracing_subscriber::registry().with(filter).with(Layer::new().with_writer(std::io::stderr)).init();
    //
    warn!("this will be ignored");
    let _ = reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO);
    info!("this will be logged");

}
