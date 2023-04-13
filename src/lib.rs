pub mod istio;

use istio::telemetries_telemetry_istio_io_v1alpha1::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference;
use kube::{api::ObjectMeta, Client};
use maplit::btreemap;
use std::collections::BTreeMap;
use std::fmt;
use tracing::*;

struct Selector<'a>(&'a BTreeMap<String, String>);

impl fmt::Display for Selector<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<String>>()
                .join(","),
        )
    }
}

pub async fn get_k8s_client() -> Result<Client, kube::Error> {
    debug!("Connecting...");
    let client = Client::try_default().await?;
    let ver = client.apiserver_version().await?;
    debug!(
        version = ver.git_version,
        platform = ver.platform,
        "Connected"
    );

    Ok(client)
}

pub fn log_telemetry_for_dep(
    dep: &str,
    provider_name: &str,
    oref: Option<OwnerReference>,
) -> Telemetry {
    Telemetry {
        metadata: ObjectMeta {
            name: Some(format!("{}-verbose-logging", dep)), // TODO: and -isolated?
            owner_references: oref.map(|or| vec![or]),
            ..ObjectMeta::default()
        },
        spec: TelemetrySpec {
            selector: Some(TelemetrySelector {
                match_labels: Some(btreemap![
                    "app".to_owned() => dep.to_owned(),
                ]),
            }),
            access_logging: Some(vec![TelemetryAccessLogging {
                r#match: Some(TelemetryAccessLoggingMatch {
                    mode: Some(TelemetryAccessLoggingMatchMode::ClientAndServer),
                }),
                providers: Some(vec![TelemetryAccessLoggingProviders {
                    name: Some(provider_name.to_owned()),
                }]),
                ..TelemetryAccessLogging::default()
            }]),
            ..TelemetrySpec::default()
        },
    }
}
