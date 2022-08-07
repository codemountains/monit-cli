use ansi_term::Colour;
use chrono::{DateTime, Local};
use clap::{ArgEnum, Parser};
use reqwest::StatusCode;
use std::process;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[clap(name = "monit", version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// URL to be monitored for response time
    url: String,

    /// Interval seconds
    #[clap(short, long, value_name = "INTERVAL SECONDS", default_value_t = 30)]
    interval: u64,

    /// Output type
    #[clap(
        short,
        long,
        arg_enum,
        value_name = "OUTPUT TYPE",
        default_value = "text"
    )]
    output: Output,
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
                    r#"{{"datetime": "{}","url: "{}", "statusCode": "{}","responseTime": "{}"}}"#,
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
    let output: Output = args.output;

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

                let color_msg = if status_code.is_success() {
                    Colour::Green.paint(&msg)
                } else {
                    Colour::Red.paint(&msg)
                };

                println!("{}", color_msg);
            }
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        }

        sleep(Duration::from_secs(interval));
    }
}
