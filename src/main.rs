mod snodas;

fn main() -> snodas::Result<()> {

    let mut connection = snodas::Connection::new()?;

    for date in snodas::days_with_data() {
        if snodas::filename_for_date(date,"data").exists() {
            println!("Skipping already downloaded {:?}", date);

            continue;
        }
        println!("Download for {:?}", date);
        connection.archive_date(date, "data")?;
    }
    return Ok(());
}