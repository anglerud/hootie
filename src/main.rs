extern crate failure;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate termion;

use std::fmt;
use std::io::{Write, stdout};
use std::{thread, time};

use failure::Error;
use structopt::StructOpt;
use termion::screen::AlternateScreen;
use termion::color;
use termion::cursor;


// TODO: sort by severity.
// TODO: Docstings
//       question - how does one doc structs?
// TODO: some proper documentation, inc /// lines.
//       question - how does one doc the module?
// TODO: Proper Readme with a screenshot.
// TODO: get rid of the unwraps in the formatting?


#[derive(StructOpt, Debug)]
#[structopt(name = "hootie", about = "Display Alerta alerts in the terminal")]
struct Opt {
    /// URL to Alerta, with query paramters. Defaults to localhost.
    #[structopt(long = "alerta-url", help = "Url to Alerta",
                default_value = "http://localhost:8080")]
    url: String,
}


#[derive(Deserialize, Debug)]
struct Alerts {
    alerts: Vec<Alert>,
}


// TODO: new Alert structure, then make this JsonAlert
//       and severity can then be an enum, time a real time, etc?
// And, implement a From or similar to automatically do a conversion?
#[derive(Deserialize, Debug)]
struct Alert {
    name: String,
    resource: String,
    severity: String,
    time: String,
}

impl fmt::Display for Alert {
    /// Custom formatter for Alert - page severity alerts are red.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg = match self.severity.as_ref() {
            "warn" => format!("{}", color::Fg(color::Yellow)),
            "page" => format!("{}", color::Fg(color::Red)),
            _ => format!("{}", color::Fg(color::White))
        };
        let reset = color::Fg(color::Reset);

        write!(f, "{}{} : {}{}", fg, self.name, self.resource, reset)
    }
}


/// Request the alerts from Alerta via http.
fn get_alerts(opt: &Opt) -> Result<Alerts, Error> {
    let mut response = reqwest::get(&opt.url)?;

    let alerts: Alerts = response.json()?;
    Ok(alerts)
}


/// Show the alerts in the terminal.
fn display(alerts: Alerts) {
    let mut screen = AlternateScreen::from(stdout());

    write!(screen, "{}Alerta alerts\n\n", cursor::Goto(0, 0)).unwrap();

    if alerts.alerts.len() == 0 {
        write!(screen, "{}No alerts, all is OK! 😌{}", color::Fg(color::Green), color::Fg(color::Reset)).unwrap();
    } else {
        for alert in alerts.alerts {
            // XXX: write! returns a result. I don't much care if it
            //      failed, what do I do? unwrap() silences it...
            write!(screen, "{}\n", alert).unwrap();
        }
    }
    screen.flush().unwrap();
}


fn main() {
    let opt = Opt::from_args();

    loop {
        let alerts_res = get_alerts(&opt);

        if let Ok(alerts) = alerts_res {
            display(alerts);
        } else {
            // TODO: Use env_logger?
            println!("ERROR: {:?}", alerts_res);
        }

        let ten_seconds = time::Duration::from_secs(10);
        thread::sleep(ten_seconds);
    }
}
