use clap::Parser;
use uuid::Uuid;
use tungstenite::connect;
use url::Url;

use std::io::Error;
use std::sync::mpsc::{self};
use std::thread;
use std::time::Duration;

mod logs;

/// Takes command as param
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Command to run
  #[arg(short, long)]
  command: String,

  /// API key to connect to log-sniffer api
  #[arg(short, long)]
  key: String,
}

fn main() -> Result<(), Error> {
  let error_code_uuid: Uuid = Uuid::new_v4();
  let args: Args = Args::parse();

  let (mut socket, _response) =
        connect(
          Url::parse(&format!("wss://hog.chameleo.dev:8080?key={}", args.key)).unwrap()
        ).expect("Can't connect to API.");

  println!("Connected to the server");
  println!("Executing: {}...", args.command);

  let (tx, rx) = mpsc::channel();

  logs::write_to_log_file(&tx, &error_code_uuid, &args.command);

  let mut lines_to_update: Vec<String>;
  'outer: loop {
    thread::sleep(Duration::from_millis(200));
    lines_to_update = Vec::new();
    for recieved in &rx {
      if recieved == String::from(error_code_uuid) {
        logs::send_logs_to_server(&mut socket, &lines_to_update);
        break 'outer;
      }
      println!("{}", recieved);
      lines_to_update.push(recieved);
    }
    logs::send_logs_to_server(&mut socket, &lines_to_update);
  }
  Ok(())
}
