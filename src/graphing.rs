use std::iter::Map;
use image::Rgba;
use image::ImageBuffer;
use std::fs::File;
use chrono::NaiveDate;
use ndarray::Array2;
use ndarray_npy::NpzReader;
use super::errors::{Result};
use super::filename_for_date;
use image::Luma;
use std::convert::TryFrom;
use image::imageops;
use image::imageops::FilterType;
use std::vec::Vec;
use image::buffer::ConvertBuffer;
use std::convert::TryInto;
use image::Pixel;
use image::GenericImage;
pub struct SnowData {
  data: Array2<i16>,
}

impl SnowData {
  pub fn for_date(date: &NaiveDate) -> Result<SnowData> {
    let mut npz = NpzReader::new(File::open(filename_for_date(date, "data"))?)?;
    Ok(
      SnowData {
        data: npz.by_name("snow_depth")?
      }
    )
  }

  pub fn from_raw(data: Array2<i16>) -> SnowData{
    SnowData {
      data: data
    }
  }

  pub fn to_image(&self, width: usize, height: usize) -> Option<ImageBuffer<Luma<u16>, Vec<u16> >> {
    let image: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(
      u32::try_from(self.data.shape()[1]).unwrap(),
      u32::try_from(self.data.shape()[0]).unwrap(),
      self.data.as_slice()?.iter().map(|p| *p as u16).collect(),
    )?;

    Some(imageops::resize(
      &image,
      u32::try_from(width).unwrap(),
      u32::try_from(height).unwrap(),
      FilterType::Triangle)
    )
  }

  pub fn display_buffer(&self, width: usize, height: usize) -> Result<Vec<u32>>{
    let image = self.to_image(width, height).unwrap();
    let i = image.pixels().map(|p| u32::from(p.channels()[0]));

    return Ok(i.collect());
  }

  pub fn width(&self) -> usize {
    self.data.shape()[1]
  }

  pub fn height(&self) -> usize {
    self.data.shape()[0]
  }
}
