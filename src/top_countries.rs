use polars::prelude::*;

pub fn get_top_ten_unicorn_countries() -> Result<DataFrame, PolarsError> {
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("D:/dev/rust/uni_n24/data/20082024_unicorn.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    // Group by country and count unicorns
    let result = df.group_by(["Country"])?.select(["Company"]).count();

    println!("{:?}", result);
    return result;
}

fn main() -> Result<(), PolarsError> {
    let top_ten = get_top_ten_unicorn_countries()?;
    println!("{:?}", top_ten);
    Ok(())
}
