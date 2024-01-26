use clap::Parser;
use uuid::Uuid;
use chrono::{Datelike, Timelike, Utc};

use std::process::{Command, Stdio, ChildStdout};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io::Write;
use std::fs::{OpenOptions, File};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn write_date_to_logs(mut log_file: &File) {
  let now: chrono::prelude::DateTime<Utc> = Utc::now();
  if let Err(e) = writeln!(
    log_file,
    "\n=============================\n[Started {}-{:02}-{:02} {:02}:{:02}:{:02}]\n=============================\n",
    now.day(), now.month(), now.year(),
    now.hour(), now.minute(), now.second()
  ) {
    eprintln!("Couldn't write to file: {}", e);
  }
}

/// Takes command as param
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Command to run
  #[arg(short, long)]
  command: String,
}

fn main() -> Result<(), Error> {
  let my_uuid: Uuid = Uuid::new_v4();
  let args: Args = Args::parse();
  println!("Executing: {}...", args.command);

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let mut log_file: File = OpenOptions::new()
      .create(true)
      .write(true)
      .append(true)
      .open("logs.log")
      .unwrap();
    write_date_to_logs(&log_file);

    let stdout: ChildStdout = Command::new("cmd")
      .arg("/C")
      .arg(&args.command)
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn().expect("")
      .stdout
      .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).expect("");

    let reader: BufReader<ChildStdout> = BufReader::new(stdout);
    reader
      .lines()
      .filter_map(|line: Result<String, Error>| line.ok())
      .for_each(|line: String| {
        if let Err(e) = writeln!(log_file, "{}", line) {
          eprintln!("Couldn't write to file: {}", e);
        }
        tx.send(line).unwrap();
      });
    tx.send(String::from(my_uuid)).unwrap();
  });

  let mut lines_to_update: Vec<String>;
  'outer: loop {
    thread::sleep(Duration::from_secs(1));
    lines_to_update = Vec::new();
    for recieved in &rx {
      if recieved == String::from(my_uuid) {
        break 'outer
      }
      println!("{}", recieved);
      lines_to_update.push(recieved);
    }
  }
  Ok(())
}




// fn send_logs_to_server(mut lines: &Vec<String>) {
//   thread::spawn(|| {
//     loop {
//       thread::sleep(Duration::from_secs(1));

//     }
//   });
// }

// fn write_date_to_logs(mut file: &File) {
//   let now: chrono::prelude::DateTime<Utc> = Utc::now();
//   if let Err(e) = writeln!(
//     file,
//     "\n=============================\n[Started {}-{:02}-{:02} {:02}:{:02}:{:02}]\n=============================\n",
//     now.day(), now.month(), now.year(),
//     now.hour(), now.minute(), now.second()
//   ) {
//     eprintln!("Couldn't write to file: {}", e);
//   }
// }

// let mut log_file: File = OpenOptions::new()
// .create(true)
// .write(true)
// .append(true)
// .open("logs.log")
// .unwrap();

// write_date_to_logs(&log_file);

// let stdout: ChildStdout = Command::new("cmd")
// .arg("/C")
// .arg(&args.command)
// .stdout(Stdio::piped())
// .stderr(Stdio::piped())
// .spawn()?
// .stdout
// .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

// let reader: BufReader<ChildStdout> = BufReader::new(stdout);

// reader
// .lines()
// .filter_map(|line: Result<String, Error>| line.ok())
// .for_each(|line: String| {
//   println!("{}", line);
//   if let Err(e) = writeln!(log_file, "{}", line) {
//     eprintln!("Couldn't write to file: {}", e);
//   }
// });


// let mut lines_to_update: Vec<String> = Vec::new();
// let delay: Duration = Duration::from_millis(10);
// let debouncer: EventDebouncer<EmptyFunc> = EventDebouncer::<fn() -> !>::new(delay, || {
//   for x in &lines_to_update {
//     println!("{x}");
//   }
// });
// debouncer.put();