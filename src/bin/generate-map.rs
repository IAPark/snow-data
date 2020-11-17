#[macro_use(s)]
extern crate ndarray;

use snodas::date_ranges::days_with_data;
use chrono::NaiveDate;
use minifb::{Key, Window, WindowOptions};

fn main() {
  let data = snodas::graphing::SnowData::for_date(&NaiveDate::from_ymd(2012, 1, 1)).unwrap();
  let width = data.width()/5;
  let height = data.height()/5;

  let mut window = Window::new(
    "Test - ESC to exit",
    width,
    height,
    WindowOptions::default(),
  )
  .unwrap_or_else(|e| {
    panic!("{}", e);
  });

  // Limit to max ~60 fps update rate
  window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

  let mut valid_day_iter = days_with_data();
  while window.is_open() && !window.is_key_down(Key::Escape) {
    let next = valid_day_iter.next();
    let date = if let Some(date) = next {
      date
    } else {
      valid_day_iter = days_with_data();
      valid_day_iter.next().unwrap()
    };
    println!("rendering {}", date);
    let data = snodas::graphing::SnowData::for_date(&date).unwrap();
    let buffer: Vec<u32> = data.display_buffer(width, height).unwrap();

    window.update_with_buffer(&buffer, width, height).unwrap();
  }
}
