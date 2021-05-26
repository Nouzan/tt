mod cli;

use anyhow::Result;
use cli::Opt;

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
    let opt = Opt::init_from_args()?;
    let root = span!(tracing::Level::DEBUG, "app_start");
    let _enter = root.enter();
    debug!("opt={:?}", opt);
    run()?;
    Ok(())
}
