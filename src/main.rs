use std::cmp::Ordering;
use std::fmt;
use std::io::{stdout, Write};
use std::{thread, time};

use anyhow::Result;
use structopt::StructOpt;
use serde::Deserialize;
use termion::color;
use termion::cursor;
use termion::screen::AlternateScreen;

// TODO: sort by severity.
//       Think this requires an enum for severity maybe
//       and then also wrapping the Json alert, *or* getting
//       serde to insert my severity enum? That seems hard.
// TODO: Docstings
//       question - how does one doc structs?
// TODO: some proper documentation, inc /// lines.
//       question - how does one doc the module?
// TODO: clear screen before first loop

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

#[derive(Deserialize, Debug)]
struct Alerts {
    alerts: Vec<Alert>,
}

#[serde(rename_all = "snake_case")]
#[derive(Deserialize, Debug)]
enum Severity {
    Page,
    Warn,
    Other
}

// TODO: new Alert structure, then make this JsonAlert
//       and severity can then be an enum, time a real time, etc?
// And, implement a From or similar to automatically do a conversion?
#[derive(Deserialize, Debug)]
struct Alert {
    event: String,
    resource: String,
    // HMM - to sort,  def need that enum I think.
    // Need to define an Eq - or maybe derive eq will just work?
    // Need a severity enum with derive Eq? or Ord?
    // TODO: get serde to deserialise to a struct
    severity: Severity
}

impl fmt::Display for Alert {
    /// Custom formatter for Alert - page severity alerts are red.
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

/// Request the alerts from Alerta via http.
fn get_alerts(opt: &Opt) -> Result<Alerts> {
    let response = reqwest::blocking::get(&opt.url)?;

    let alerts: Alerts = response.json()?;
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
