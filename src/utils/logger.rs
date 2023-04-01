use anyhow::Result;
use time::macros::format_description;
use tracing::Level;
use tracing_subscriber::{fmt::{writer::MakeWriterExt, time::LocalTime, Layer}, prelude::__tracing_subscriber_SubscriberExt};

// Sets up a global logger for the whole application
pub fn setup_logging() -> Result<()> {

    // TODO: Get the max logging level
    // let log_level = match 

    // TODO: 
    // Setup the subscriber for stdout
    let stdout = std::io::stdout.with_max_level(Level::DEBUG);
    
    let time_format = LocalTime::new(format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]"));

    let subscriber = tracing_subscriber::registry()
        .with(Layer::new()
            .with_writer(stdout)
            .with_timer(time_format)
    );

    tracing::subscriber::set_global_default(subscriber).expect("failed to set global logging instance");

    Ok(())
}
