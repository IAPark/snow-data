use snodas;

fn main() -> snodas::Result<()> {

    let mut connection = snodas::Connection::new()?;
    let mut failures_in_row = 0;

    for date in snodas::date_ranges::days_with_data() {
        if snodas::filename_for_date(&date,"data").exists() {
            //println!("Skipping already downloaded {:?}", date);
            failures_in_row=0;
            continue;
        }
        println!("Downloading for {:?}", date);
        if let Err(e) = connection.archive_date(date, "data") {
            if failures_in_row > 5 {
                return Err(e)
            }

            println!("Error for {:?}: {}", date, e);
            failures_in_row+=1;
        } else {
            failures_in_row=0;
        }
    }
    return Ok(());
}