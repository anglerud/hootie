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

use color_eyre::eyre::{WrapErr, Result};
use structopt::StructOpt;
use serde::Deserialize;
use termion::color;
use termion::cursor;
use termion::screen::AlternateScreen;

static COMPRESSED_DEPENDENCY_LIST: &[u8] = auditable::inject_dependency_list!();

// TODO: Use env_logger (or similar) instead of println!
// Just for delta

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
#[derive(Clone, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Severity {
    Page,
    Warn,
    Other
}

/// Alert represents one individual alert.
#[derive(Clone, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
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
    let response = reqwest::blocking::get(&opt.url)
        .wrap_err("Could not contact Alerta.")?;

    let alerts: Alerts = response.json()
        .wrap_err("Could not understand the response from Alerta.")?;
    Ok(alerts)
}

/// Show the alerts in the terminal.
fn display(alerts: Alerts) -> std::io::Result<()> {
    let mut screen = AlternateScreen::from(stdout());

    write!(screen, "{}{}Alerta alerts\n\n", termion::clear::All, cursor::Goto(1, 1))?;

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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_alert_sort_order() {
        // Two alerts with the same resource and events
        // We check that the severity sorts one above the
        // other.
        let crit_alert = Alert{
            severity: Severity::Page,
            resource: "A".into(),
            event: "A".into()
        };
        let warn_alert = Alert{
            severity: Severity::Warn,
            resource: "A".into(),
            event: "A".into()
        };
        let crit_alert_2 = crit_alert.clone();
        let warn_alert_2 = warn_alert.clone();
        let mut alerts = vec![warn_alert, crit_alert];
        alerts.sort();

        let expected = vec![crit_alert_2, warn_alert_2];
        assert_eq!(alerts, expected);
    }

    #[test]
    fn test_page_alert_format() {
        // [38;5;<color>m - set foreground color
        // In 256 color mode, color 1 is red.
        // \u{1b}[39m - reset colors.
        // And our expected value is "A : A"
        let expected = "\u{1b}[38;5;1mA : A\u{1b}[39m";

        let crit_alert = Alert{
            severity: Severity::Page,
            resource: "A".into(),
            event: "A".into()
        };

        assert_eq!(&format!("{}", crit_alert), expected);
    }

}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    println!("{}", COMPRESSED_DEPENDENCY_LIST[0]);

    loop {
        let alerts_res = get_alerts(&opt);

        if let Ok(mut alerts) = alerts_res {
            alerts.alerts.sort();
            display(alerts)?;
        } else {
            println!("ERROR: {:?}", alerts_res);
        }

        let ten_seconds = time::Duration::from_secs(10);
        thread::sleep(ten_seconds);
    }
}
