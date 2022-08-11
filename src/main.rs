use ansi_term::Colour;
use chrono::{DateTime, Local};
use clap::{ArgEnum, Parser};
use reqwest::StatusCode;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs, process};

#[derive(Parser, Debug)]
#[clap(name = "monit", version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// URL to be monitored for response time
    url: String,

    /// Interval seconds
    #[clap(short, long, value_name = "INTERVAL SECONDS", default_value_t = 30)]
    interval: u64,

    /// Bolder seconds for alert
    #[clap(short, long, value_name = "BOLDER SECONDS", default_value_t = 3)]
    bolder: u64,

    /// Output type
    #[clap(
        short,
        long,
        arg_enum,
        value_name = "OUTPUT TYPE",
        default_value = "text"
    )]
    output: Output,

    /// File path for output
    #[clap(short, long, value_name = "FILE PATH")]
    file: Option<PathBuf>,

    /// Send request only once
    #[clap(short = 'O', long)]
    one_time: bool,
}

#[derive(Debug, Clone, ArgEnum, Copy)]
enum Output {
    Csv,
    Json,
    Text,
}

struct OutputMessage {
    datetime: DateTime<Local>,
    url: String,
    status_code: StatusCode,
    elapsed: Duration,
}

impl OutputMessage {
    fn new(
        datetime: DateTime<Local>,
        url: String,
        status_code: StatusCode,
        elapsed: Duration,
    ) -> Self {
        Self {
            datetime,
            url,
            status_code,
            elapsed,
        }
    }

    fn to_formatted(&self, output: Output) -> String {
        let dt = self.datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        let url = self.url.as_str().to_string();
        let st = self.status_code.to_string();
        let response_time = format!(
            "{}.{:03}",
            self.elapsed.as_secs(),
            self.elapsed.subsec_nanos() / 1_000_000
        );

        match output {
            Output::Csv => {
                format!(r#""{}","{}","{}","{}""#, dt, url, st, response_time)
            }
            Output::Json => {
                format!(
                    r#"{{"datetime": "{}","url: "{}","statusCode": "{}","responseTime": "{}"}}"#,
                    dt, url, st, response_time
                )
            }
            Output::Text => {
                format!("{} {} {} {}", dt, url, st, response_time)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // comment in on Windows 10
    // let _ = ansi_term::enable_ansi_support();

    let args = Args::parse();
    let url: &str = &args.url;
    let interval: u64 = args.interval;
    let border: u64 = args.bolder;
    let output: Output = args.output;
    let file: Option<PathBuf> = args.file;
    let is_one_time: bool = args.one_time;

    if let Some(file_path) = &file {
        if let Some(parent_path) = file_path.parent() {
            fs::create_dir_all(parent_path).unwrap_or_else(|err| {
                eprintln!("{}", err);
                process::exit(1);
            });
        }
    }

    loop {
        let dt_now = Local::now();

        let start = Instant::now();
        let resp = reqwest::get(url).await;
        let end = start.elapsed();

        match resp {
            Ok(resp) => {
                let status_code = resp.status();

                let output_msg = OutputMessage::new(dt_now, url.to_string(), status_code, end);
                let msg = output_msg.to_formatted(output);

                if let Some(file_path) = &file {
                    if let Err(err) = write_file(file_path, &msg) {
                        eprintln!("{}", err);
                        process::exit(1);
                    }
                }

                let color_msg = if !status_code.is_success() || end.as_secs() >= border {
                    Colour::Red.paint(&msg)
                } else {
                    Colour::Green.paint(&msg)
                };

                println!("{}", color_msg);
            }
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        }

        if is_one_time {
            process::exit(0);
        }

        sleep(Duration::from_secs(interval));
    }
}

fn write_file(file_path: &PathBuf, msg: &str) -> Result<(), std::io::Error> {
    let output_msg = msg.to_string() + "\n";

    let output_file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open(file_path);

    match output_file {
        Ok(mut file) => {
            file.write_all(output_msg.as_bytes())?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}
