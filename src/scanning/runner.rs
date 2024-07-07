use crate::scanning::portscan::{scan_common_tcp_ports, scan_ports_for_target, Target};
use log::{debug, info};
use std::collections::HashSet;
use std::error::Error;
use std::time::Duration;

pub async fn run(target: Target) -> Result<(), Box<dyn Error>> {
    let mut ports_to_scan = HashSet::from_iter(target.start_port..=target.end_port);
    // TODO: Make these configurable
    let durations = &[
        (Duration::new(0, 25_000_000), Duration::new(1, 0)),
        (Duration::new(0, 50_000_000), Duration::new(2, 0)),
        (Duration::new(0, 100_000_000), Duration::new(5, 0)),
    ];
    info!("Rmap has started.");
    info!(
        "Scanning target {} over {} on ports {}-{}",
        target.hostname, target.proto, target.start_port, target.end_port
    );
    info!("Starting scan for common ports");
    let common_duration = Duration::new(0, 200_000_000);
    let common_sleep = Duration::new(5, 0);
    ports_to_scan =
        match scan_common_tcp_ports(&target, &ports_to_scan, common_duration, common_sleep).await {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
    // Here we make sure we don't scan the already found ports.
    for (sleep_duration, timeout_duration) in durations {
        info!(
            "Scanning with {} probes per second",
            1_000_000_000 / sleep_duration.as_micros()
        );
        let cloned_target = target.clone();
        let cloned_sleep = *sleep_duration;
        let cloned_timeout = *timeout_duration;
        ports_to_scan = match scan_ports_for_target(
            cloned_target,
            &ports_to_scan,
            cloned_sleep,
            cloned_timeout,
        )
        .await
        {
            Ok(p) => p,
            Err(e) => {
                debug!("Portscan returned with error: {}.", e);
                return Err(e.into());
            }
        };
        if ports_to_scan.is_empty() {
            info!("Rmap determined the state OPEN or CLOSED for all ports. No ports with state UNKNOWN.");
            return Ok(());
        }
        info!(
            "{} ports were state: UNKNOWN and need to be scanned again.",
            ports_to_scan.len(),
        );
        debug!(
            "{} ports {:?} are state UNKNOWN.",
            target.proto, ports_to_scan
        );
    }
    Ok(())
}
