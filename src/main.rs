use std::io::{self};

mod reader;

fn main() -> io::Result<()> {
  let device = rodio::default_output_device().unwrap();
  let mut sink = rodio::Sink::new(&device);

  loop {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd)?;

    match cmd.trim_end() {
      "play" => {
        let mut url = String::new();
        io::stdin().read_line(&mut url)?;
        let buffer = reader::WebStreamReader::new(url.trim());
        let source = rodio::Decoder::new(buffer).unwrap();
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
        sink = rodio::Sink::new(&device);
      }
      "volume" => {
        let mut vol = String::new();
        io::stdin().read_line(&mut vol)?;
        sink.set_volume(vol.trim_end().parse::<f32>().unwrap() / 100.0);
      }
      _ => ()
    }
  }
}
