use std::path::PathBuf;
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
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::collections::HashSet;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct ShippableDateRange {
  start: NaiveDate,
  end: NaiveDate,
  skipping: HashSet<NaiveDate>,
}
impl ShippableDateRange {
  pub fn new(start: NaiveDate, end: NaiveDate, skipping: HashSet<NaiveDate>) -> ShippableDateRange {
    ShippableDateRange {
      start: start,
      end: end,
      skipping: skipping,
    }
  }
}

impl std::iter::Iterator for ShippableDateRange {
  fn next(&mut self) -> std::option::Option<NaiveDate> {
    if self.start < self.end {
      let old_start = self.start;
      self.start += Duration::days(1);

      if self.skipping.contains(&old_start) {
        return self.next()
      }
      return Some(old_start)
    } else {
      return None
    }
  }
  type Item = NaiveDate;
}

pub fn days_with_data() -> ShippableDateRange {
  return ShippableDateRange::new(
    NaiveDate::from_ymd(2003, 9, 30),
    Utc::now().naive_utc().date(),
    days_without_data(),
  )
}

fn days_without_data() -> HashSet<NaiveDate> {
  let mut set = HashSet::new();
  set.insert(NaiveDate::from_ymd(2004, 2, 25)); // there seems to be nothing on this day
  set.insert(NaiveDate::from_ymd(2004, 8, 31)); // I wonder if they just forgot this was a day
  set.insert(NaiveDate::from_ymd(2004, 9, 27)); // I wonder if they just forgot this was a day
  set.insert(NaiveDate::from_ymd(2005, 6, 25)); // I wonder if they just forgot this was a day
  set.insert(NaiveDate::from_ymd(2005, 8, 1)); // I wonder if they just forgot this was a day
  set.insert(NaiveDate::from_ymd(2005, 8, 2)); // I wonder if they just forgot this was a day
  set.insert(NaiveDate::from_ymd(2006, 8, 25));
  set.insert(NaiveDate::from_ymd(2006, 8, 26));
  set.insert(NaiveDate::from_ymd(2006, 8, 27));
  set.insert(NaiveDate::from_ymd(2006, 9, 8));
  set.insert(NaiveDate::from_ymd(2006, 9, 30));
  set.insert(NaiveDate::from_ymd(2006, 10, 1));
  set.insert(NaiveDate::from_ymd(2007, 2, 14));
  set.insert(NaiveDate::from_ymd(2007, 3, 26));
  set.insert(NaiveDate::from_ymd(2008, 3, 13));
  set.insert(NaiveDate::from_ymd(2008, 6, 13));
  set.insert(NaiveDate::from_ymd(2008, 6, 18));
  set.insert(NaiveDate::from_ymd(2009, 8, 20));
  set.insert(NaiveDate::from_ymd(2012, 12, 20));

  return set;
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
    let filename = filename_for_date(date, archive_prefix);

    let data = self.get_data(date)?;
    let mut npz = NpzWriter::new_compressed(File::create(filename)?);
    npz.add_array("snow_depth", &data)?;

    return Ok(())
  }
}

pub fn filename_for_date(date: NaiveDate, archive_prefix: &str) -> PathBuf {
  return PathBuf::from(date
      .format(&(archive_prefix.to_owned() + "/%Y%m%d_snow_depth.npz"))
      .to_string());
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

    let reader = ftp_stream.get(&file_path);
    let mut archive = Archive::new(reader?);
    let result = (||{
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
    })();

    drop(archive);
    ftp_stream.read_response_in(&[450, 226])?;

    ftp_stream.get_ref().write(b"ABOR\r\n")?;
    ftp_stream.read_response(226)?;
    return result
}