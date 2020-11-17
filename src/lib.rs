pub mod date_ranges;
pub mod connection;
pub mod errors;


use std::path::PathBuf;
use chrono::NaiveDate;
pub use connection::Connection;
pub use errors::Result;

pub fn filename_for_date(date: NaiveDate, archive_prefix: &str) -> PathBuf {
  return PathBuf::from(date
      .format(&(archive_prefix.to_owned() + "/%Y%m%d_snow_depth.npz"))
      .to_string());
}
