use tungstenite::stream::MaybeTlsStream;
use uuid::Uuid;
use chrono::{Datelike, Timelike, Utc};
use tungstenite::{Message, WebSocket};

use std::net::TcpStream;
use std::process::{Command, Stdio, ChildStdout};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io::Write;
use std::fs::{OpenOptions, File};
use std::sync::mpsc::Sender;
use std::thread;
use std::env;

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

pub fn write_to_log_file(
    tx_ref: &Sender<String>,
    error_code_uuid_ref: &Uuid, 
    command_ref: &String
) {
  let error_code_uuid = error_code_uuid_ref.clone();
  let command = command_ref.clone();
  let tx = tx_ref.clone();

  // let mut log_file: File = OpenOptions::new()
  //   .create(true)
  //   .write(true)
  //   .append(true)
  //   .open("logs.log")
  //   .unwrap();

  // write_date_to_logs(&log_file);

  // let mut child = Command::new("cmd")
  //   .arg("/C")
  //   .arg(&command)
  //   .stdout(Stdio::piped())
  //   .stderr(Stdio::piped())
  //   .spawn()
  //   .unwrap();

  // let child_stdout = child
  //   .stdout
  //   .take()
  //   .expect("Could not take stdout");

  // let child_stderr = child
  //   .stderr
  //   .take()
  //   .expect("Could not take stdout");

  // thread::spawn(move || {

  // });
  thread::spawn(move || {
    let mut log_file: File = OpenOptions::new()
      .create(true)
      .write(true)
      .append(true)
      .open("logs.log")
      .unwrap();

    write_date_to_logs(&log_file);

    let stdout: ChildStdout;
    if cfg!(target_os = "windows") {
        stdout = Command::new("cmd")
            .arg("/C")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().expect("")
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).expect("");
    } else {
        stdout = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().expect("")
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).expect("");
    }

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
    tx.send(String::from(error_code_uuid)).unwrap();
  });
}

pub fn send_logs_to_server(
  socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
  lines_to_update: &Vec<String>
) {
  let combined_message = lines_to_update.join("\n");
  println!("Sending: {}", combined_message);
  socket.send(Message::Text(combined_message.into())).unwrap();
}
