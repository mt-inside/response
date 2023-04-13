use clap::Parser;
use tracing::*;
use tracing_subscriber::{filter, prelude::*};

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(author = "Matt Turner")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generates Istio config for cloud-native incident responses", long_about = None)]
struct Args {
    #[arg(short, long)]
    kubeconfig: Option<String>,
    #[arg(short, long)]
    pod_name: String,
    #[arg(short, long)]
    dep_name: String,
}

// TODO:
// - don't actually need to call kube here? Can all be a priori?
// - should read CRD from disk as input, but cli args for now

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.kubeconfig.is_some() {
        panic!("Don't support alternate kubeconfig location yet");
    };

    tracing_subscriber::registry()
        .with(
            filter::Targets::new()
                .with_target("response", Level::TRACE)
                .with_target("response_generator", Level::TRACE),
        ) //off|error|warn|info|debug|trace
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    info!(version = %version_check::Version::read().unwrap(), "Rustc");

    let tele = response::log_telemetry_for_dep(&args.dep_name, "envoy-verbose-log", None);
    println!("{}", serde_yaml::to_string(&tele)?);
    println!("---");

    Ok(())
}
