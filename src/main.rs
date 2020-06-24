use std::io::{self};

mod reader;

fn main() -> io::Result<()> {
  let device = rodio::default_output_device().unwrap();
  let sink = rodio::Sink::new(&device);

  loop {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd)?;

    match cmd.trim_end() {
      "play" => {
        let mut url = String::new();
        io::stdin().read_line(&mut url)?;
        let buffer = reader::WebStreamReader::new(url.trim());
        let source = rodio::Decoder::new(buffer).unwrap();
        sink.stop();
        sink.append(source);
      }
      "pause" => {
        sink.pause();
      }
      "resume" => {
        sink.play();
      }
      "stop" => {
        sink.stop();
      }
      _ => {
        println!("{}", &cmd);
      }
    }
  }
}
