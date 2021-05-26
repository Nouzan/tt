use clap_verbosity_flag::Verbosity;
use log::Level;
use structopt::StructOpt;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// The config of the application.
#[derive(Debug, StructOpt)]
pub(crate) struct Config {}

#[derive(Debug, StructOpt)]
pub(crate) struct Opt {
    #[structopt(flatten)]
    verbose: Verbosity,

    #[structopt(flatten)]
    pub config: Config,
}

impl Opt {
    #[instrument]
    pub(crate) fn init_from_args() -> anyhow::Result<Self> {
        let mut opt = Opt::from_args();

        // set default log level.
        opt.verbose.set_default(Some(Level::Info));

        let level = match opt.verbose.log_level() {
            Some(Level::Trace) => LevelFilter::TRACE,
            Some(Level::Debug) => LevelFilter::DEBUG,
            Some(Level::Info) => LevelFilter::INFO,
            Some(Level::Warn) => LevelFilter::WARN,
            Some(Level::Error) => LevelFilter::ERROR,
            None => LevelFilter::OFF,
        };
        let fmt = tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(false)
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
            .compact();
        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("report_example")
            .install_simple()?;
        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(opentelemetry)
            .with(level)
            .with(fmt)
            .try_init()?;
        Ok(opt)
    }
}
