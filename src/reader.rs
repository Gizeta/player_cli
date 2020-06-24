use std::io::{BufRead, Error, ErrorKind, Read, Seek, SeekFrom};
use reqwest::blocking::{get, Response};

pub struct WebStreamReader {
  inner: Response,
  position: usize,
  buffer: Vec<u8>,
}

impl WebStreamReader {
  pub fn new(url: &str) -> Self {
    WebStreamReader {
      inner: get(url).unwrap(),
      position: 0,
      buffer: Vec::default(),
    }
  }

  fn slice(&mut self, from: usize, to: usize) -> &[u8] {
    if self.buffer.len() <= to {
      self.buffer_to_index(to);
    }
    if self.buffer.len() <= from {
      return &[];
    }
    if self.buffer.len() <= to {
      return &self.buffer[from..];
    }
    &self.buffer[from..to]
  }

  fn buffer_to_index(&mut self, index: usize) {
    let available = self.buffer.len();
    if index >= available {
      let mut buf = vec![0u8; index - available];
      self.inner.read(&mut buf).map(|len| {
        self.buffer.extend(&buf[..len]);
        len
      }).unwrap();
    }
  }
}

impl Read for WebStreamReader {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
    let position = self.position;
    let data = self.slice(position, position + buf.len());
    let amt = data.len();
    buf[..amt].copy_from_slice(data);
    self.consume(amt);
    Ok(amt)
  }
}

impl Seek for WebStreamReader {
  fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
    let (position, offset) = match pos {
      SeekFrom::Start(position) => (0, position as i64),
      SeekFrom::Current(position) => (self.position, position),
      SeekFrom::End(position) => (self.buffer.len(), position),
    };
    let position = if offset < 0 {
      position.checked_sub(offset.wrapping_neg() as usize)
    } else {
      position.checked_add(offset as usize)
    };
    match position {
      Some(position) => {
        self.position = position;
        Ok(position as u64)
      }
      None => Err(Error::new(
        ErrorKind::InvalidInput,
        "invalid seek to a negative or overflowing position",
      ))
    }
  }
}

impl BufRead for WebStreamReader {
  fn fill_buf(&mut self) -> Result<&[u8], Error> {
    Ok(self.slice(self.position, self.buffer.len()))
  }

  fn consume(&mut self, amt: usize) {
    self.seek(SeekFrom::Current(amt as i64)).unwrap();
  }
}
