use chrono::NaiveDate;
use chrono::Duration;
use std::collections::HashSet;
use chrono::Utc;


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