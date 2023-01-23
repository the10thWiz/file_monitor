use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use clap::Parser;
use notify::{Event, EventKind, RecursiveMode, Watcher};

/// File & Directory Monitor for Linux
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Opts {
    /// Path to monitor
    path: PathBuf,
    /// Min Time to print a second change event
    #[arg(short, long)]
    min_time: Duration,
    /// Whether to attempt to print to every
    #[arg(short, long, default_value)]
    wall: bool,
}

fn main() -> anyhow::Result<()> {
    let min_time_diff = Duration::from_secs(3);
    let path = Path::new(".");

    let mut last = Instant::now() - min_time_diff;
    let mut file = File::options().append(true).open("/var/log/file_monitor")?;
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(Event {
            kind: EventKind::Access(_),
            ..
        }) => (),
        Ok(Event { paths, .. }) => {
            if last.elapsed() > min_time_diff {
                // notify
                println!("File Modification Occurred");
                let _ = Command::new("wall")
                    .arg("File Modification Occurred")
                    .spawn();
                last = Instant::now();
            }
            for path in paths {
                let _ = writeln!(file, "{:?}", path);
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    thread::spawn(move || {
        let mut file = Command::new("sha256sum")
            .arg("-c")
            .arg(path)
            .stdin(Stdio::piped())
            .spawn()?;
        std::io::copy(
            &mut File::open("/ccdc/log/www_hashes")?,
            file.stdin.as_mut().unwrap(),
        )?;
        let status = file.wait()?;
        if !status.success() {
            let _ = Command::new("wall")
                .arg("File Modification Occurred")
                .spawn();
        }
        Ok(()) as std::io::Result<()>
    });
    watcher.watch(path, RecursiveMode::Recursive)?;
    Ok(())
}
