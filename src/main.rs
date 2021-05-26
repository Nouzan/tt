mod cli;

use anyhow::Result;
use cli::Opt;
use opentelemetry::global;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate tracing_attributes;

#[instrument]
fn run() -> Result<()> {
    info!("Hello, world!");
    Ok(())
}

fn main() -> Result<()> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let opt = Opt::init_from_args()?;
    let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    let _enter = root.enter();
    debug!("opt={:?}", opt);
    run()?;
    global::shutdown_tracer_provider();
    Ok(())
}
