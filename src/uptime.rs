use std::sync::OnceLock;

use crate::{Error, Result};

const UPTIME_PATH: &str = "/proc/uptime";

fn read_system_uptime() -> Result<std::time::Duration> {
    let buffer = std::fs::read_to_string(UPTIME_PATH)
        .map_err(|x| Error(format!("Error reading from {UPTIME_PATH}: {x}")))?;
    let line = buffer
        .lines()
        .next()
        .ok_or_else(|| Error(format!("Could not read from {UPTIME_PATH}.")))?;
    let uptime: f64 = crate::get_number(
        &format!("Could not read uptime from {UPTIME_PATH}"),
        line,
        0,
    )?;
    Ok(std::time::Duration::from_secs_f64(uptime))
}

static START_TIME: OnceLock<std::time::Instant> = OnceLock::new();
static START_SYSTEM_UPTIME: OnceLock<std::time::Duration> = OnceLock::new();

pub fn system() -> std::time::Duration {
    *START_SYSTEM_UPTIME.get().unwrap() + process()
}

pub fn process() -> std::time::Duration {
    std::time::Instant::now().duration_since(*START_TIME.get().unwrap())
}

pub fn init() -> Result<()> {
    _ = START_TIME.set(std::time::Instant::now());

    let system_uptime = read_system_uptime();
    match system_uptime {
        Ok(system_uptime) => {
            _ = START_SYSTEM_UPTIME.set(system_uptime);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
