use std::fs::File;
use ndarray_npy::NpzWriter;
use ndarray::Array;
use ndarray::Ix2;
use std::io::BufReader;
use std::io::prelude::*;
use ftp::FtpStream;
use chrono::NaiveDate;
use tar::Archive;
use flate2::read::GzDecoder;
use std::convert::TryInto;

use super::errors::{Result,MissingDataError};
use super::filename_for_date;

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
    let filename = filename_for_date(&date, archive_prefix);

    let data = self.get_data(date)?;
    let mut npz = NpzWriter::new_compressed(File::create(filename)?);
    npz.add_array("snow_depth", &data)?;

    return Ok(())
  }
}



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