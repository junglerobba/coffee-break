use std::{
    io::{self, BufRead, Write},
    process::Command,
    str::FromStr,
};

use anyhow::{Error, Result};
use chrono::{DateTime, Duration, Local, NaiveTime, TimeDelta};
use clap::{command, Parser};
use duration_string::DurationString;
use flags::CaffeinateFlags;
use fork::{daemon, Fork};
use sysinfo::Process;

mod flags;

#[derive(Parser, Debug)]
struct Cli {
    /// How long to caffeinate. Accepts either a timestamp (18:00) or a duration (8h)
    time: Option<String>,
    #[command(flatten)]
    flags: CaffeinateFlags,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let now = Local::now();

    let mut system = sysinfo::System::new();
    system.refresh_all();

    for p in system.processes_by_exact_name("caffeinate") {
        if let Some(time) = get_active_caffeinate_time(p) {
            println!("Already caffeinating until {}", time);
            let flags: CaffeinateFlags = p.into();
            if flags.any() {
                println!("Currently suspending {:#}", flags);
            } else {
                println!("Currently not suspending anything");
            }
            let msg = match args.time {
                Some(_) => "Kill existing and get fresh coffee instead? [y/N] ",
                None => "Throw away some perfectly good coffee? [y/N] ",
            };
            print!("{}", msg);
            io::stdout().flush()?;
            let stdin = io::stdin();
            let mut line = String::new();
            stdin.lock().read_line(&mut line)?;

            if line.trim().to_lowercase() == "y" {
                p.kill();
            } else {
                return Ok(());
            }
        }
    }
    let Some(time) = args.time else { return Ok(()) };

    let target = try_time(&time, &now).or_else(|_| try_duration(&time, &now))?;

    println!("☕️ Caffeinating until {}", target.to_rfc2822());

    let diff = target - now;
    let diff = diff.num_seconds();

    let flags = args.flags;

    if let Ok(Fork::Child) = daemon(false, false) {
        let mut child = Command::new("/usr/bin/caffeinate");
        if flags.any() {
            child.arg(format!("-{flags}"));
        }
        child.arg("-t").arg(diff.to_string());
        child.spawn()?.wait()?;
    }
    Ok(())
}

fn try_time(input: &str, now: &DateTime<Local>) -> Result<DateTime<Local>, Error> {
    let time = NaiveTime::from_str(input)?;
    let mut now = *now;
    if now.time().gt(&time) {
        now += Duration::days(1);
    }
    let target = now.with_time(time).unwrap();

    Ok(target)
}

fn try_duration(input: &str, now: &DateTime<Local>) -> Result<DateTime<Local>, Error> {
    let duration: std::time::Duration = DurationString::try_from(String::from(input))?.into();
    let duration = TimeDelta::from_std(duration)?;
    let target = now.checked_add_signed(duration).unwrap();

    Ok(target)
}

fn get_active_caffeinate_time(process: &Process) -> Option<DateTime<Local>> {
    let start_time = DateTime::from_timestamp(process.start_time() as i64, 0)?;
    let start_time: DateTime<Local> = DateTime::from(start_time);
    let index = process.cmd().iter().position(|arg| *arg == "-t")?;
    let seconds: i64 = process.cmd().get(index + 1)?.parse().ok()?;
    let seconds = TimeDelta::seconds(seconds);
    Some(start_time + seconds)
}
