//! Hootie - the terminal view of Alerta alerts.
//!
//! See your alerts in the terminal. An example of Hootie
//! in action:
//!
//! ![Hootie's main view](https://github.com/anglerud/hootie/raw/main/hootie.png)
//!
//! Usage:
//!
//!     hootie --alerta-url="http://your-alerta-url/api/alerts?status=open&service=infra"
//!
//! Note that you can select things like status and service via the URL to filter
//! for the alerts you're interested in.
use std::fmt;
use std::io::{stdout, Write};
use std::{thread, time};

use anyhow::Result;
use structopt::StructOpt;
use serde::Deserialize;
use termion::color;
use termion::cursor;
use termion::screen::AlternateScreen;

// TODO: test the sort by severity.
// TODO: clear screen before first loop

/// Configuration struct
///
/// StructOpt struct containig Hootie's command line parameters.
#[derive(StructOpt, Debug)]
#[structopt(name = "hootie", about = "Display Alerta alerts in the terminal")]
struct Opt {
    /// URL to Alerta, with query paramters. Defaults to localhost.
    #[structopt(
        long = "alerta-url",
        help = "Url to Alerta",
        default_value = "http://localhost:8080"
    )]
    url: String,
}

/// Alerts is the JSON struct returned from the Alerta API.
/// Importantly, it contains a list of Alert.
#[derive(Deserialize, Debug)]
struct Alerts {
    alerts: Vec<Alert>,
}

/// Alert severities - most severe first.
#[serde(rename_all = "snake_case")]
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Severity {
    Page,
    Warn,
    Other
}

/// Alert is the JSON struct returned from the Alerta API
/// that represents an individual alert.
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Alert {
    severity: Severity,
    event: String,
    resource: String
}

/// Custom formatter for Alert - page severity alerts are red,
/// warnings yellow.
impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg = match self.severity {
            Severity::Page => format!("{}", color::Fg(color::Red)),
            Severity::Warn => format!("{}", color::Fg(color::Yellow)),
            _ => format!("{}", color::Fg(color::White)),
        };
        let reset = color::Fg(color::Reset);

        write!(f, "{}{} : {}{}", fg, self.event, self.resource, reset)
    }
}

/// Request alerts from Alerta via an http GET.
fn get_alerts(opt: &Opt) -> Result<Alerts> {
    // FIXME: how to test this - create a trait for getting a url?
    // 
    let response = reqwest::blocking::get(&opt.url)?;

    let mut alerts: Alerts = response.json()?;
    alerts.alerts.sort();
    Ok(alerts)
}

/// Show the alerts in the terminal.
fn display(alerts: Alerts) -> std::io::Result<()> {
    let mut screen = AlternateScreen::from(stdout());

    write!(screen, "{}Alerta alerts\n\n", cursor::Goto(1, 1))?;

    if alerts.alerts.is_empty() {
        write!(
            screen,
            "{}No alerts, all is OK! 😌{}",
            color::Fg(color::Green),
            color::Fg(color::Reset)
        )
        .unwrap();
    } else {
        for alert in alerts.alerts {
            writeln!(screen, "{}", alert)?;
        }
    }
    screen.flush()
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    loop {
        let alerts_res = get_alerts(&opt);

        if let Ok(alerts) = alerts_res {
            display(alerts)?;
        } else {
            // TODO: Use env_logger?
            println!("ERROR: {:?}", alerts_res);
        }

        let ten_seconds = time::Duration::from_secs(10);
        thread::sleep(ten_seconds);
    }
}
