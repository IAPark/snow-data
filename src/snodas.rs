use chrono::Utc;
use std::fs::File;
use ndarray_npy::NpzWriter;
use ndarray::Array;
use ndarray::Ix2;
use std::io::BufReader;
use std::io::prelude::*;
use ftp::FtpStream;
use chrono::NaiveDate;
use chrono::Duration;
use tar::Archive;
use flate2::read::GzDecoder;
use std::net::Shutdown;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct DateRange {
  start: NaiveDate,
  end: NaiveDate,
}
impl DateRange {
  pub fn new(start: NaiveDate, end: NaiveDate) -> DateRange {
    DateRange {
      start: start,
      end: end,
    }
  }
}

impl std::iter::Iterator for DateRange {
  fn next(&mut self) -> std::option::Option<NaiveDate> {
    if self.start < self.end {
      let old_start = self.start;
      self.start += Duration::days(1);
      return Some(old_start)
    } else {
      return None
    }
  }
  type Item = NaiveDate;
}

pub fn days_with_data() -> DateRange {
  return DateRange::new(
    NaiveDate::from_ymd(2003, 9, 30),
    Utc::now().naive_utc().date()
  )
}


#[derive(Debug)]
pub struct Connection {
  ftp: FtpStream
}

impl Connection {
  pub fn new() -> Result<Connection> {
    let mut ftp = FtpStream::connect("sidads.colorado.edu:21")?;
    ftp.login("","")?;
    ftp.transfer_type(ftp::types::FileType::Binary)?;

    Ok(Connection {
      ftp: ftp
    })
  }

  pub fn get_data(&mut self, date: NaiveDate) -> Result<Array<i16, Ix2>> {
    extract_grid(&mut self.ftp, date)
  }

  pub fn archive_date(&mut self, date: NaiveDate, archive_prefix: &str) -> Result<()> {
    let filename = date
      .format(&(archive_prefix.to_owned() + "/%Y%m%d_snow_depth.npz"))
      .to_string();

    let mut npz = NpzWriter::new(File::create(filename)?);
    let data = self.get_data(date)?;
    npz.add_array("snow_depth", &data)?;

    return Ok(())
  }
}


#[derive(Debug, Clone,)]
struct MissingDataError;

impl fmt::Display for MissingDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MissingDataError")
    }
}
impl Error for MissingDataError {}


fn extract_grid(ftp_stream: &mut FtpStream, date: NaiveDate)
        -> Result<Array<i16, Ix2>> {
    let file_path = date.format("/DATASETS/NOAA/G02158/masked/%Y/%m_%b/SNODAS_%Y%m%d.tar")
        .to_string();
    println!("path to download {:?}", file_path);
    let reader = ftp_stream.simple_retr(&file_path)?;
    let mut archive = Archive::new(reader);
    let entry_path = date.format("us_ssmv11036tS__T0001TTNATS%Y%m%d05HP001.dat.gz").to_string();

    for entry in archive.entries().unwrap() {
        let entry = entry.unwrap();
        let path = entry.header().path().unwrap();
        if path.to_str().unwrap() == entry_path {
            let mut decoder = GzDecoder::new(BufReader::new(entry));
            let mut data = Vec::new();
            let _ = decoder.read_to_end(&mut data);
            let data: Vec<i16> = data.chunks_exact(2)
                .map(|bytes| i16::from_be_bytes(bytes.try_into().unwrap()))
                .collect();

            let grid = Array::from_shape_vec((3351, 6935), data)?;
            return Ok(grid)
        }
    }
    return Err(MissingDataError.into())
}