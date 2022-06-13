use chrono::NaiveDate;
use arrow2_convert::ArrowField;
use rayon::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, ArrowField)]
#[serde(rename_all = "PascalCase")]
struct StockBar
{
   date: NaiveDate,
   low: f64,
   open: f64,
   volume: f64,
   high: f64,
   close: f64,
   #[serde(rename="Adjusted Close")]
   adjusted_close: f64,
}

fn main() {
    let stock_bars: Result<Vec<StockBar>, anyhow::Error> = env::args().collect::<Vec<String>>().par_iter().map(|a| -> anyhow::Result<StockBar> {
        let ret: Vec<StockBar> = vec![];
        let file = File::open("foo.txt")?;
        let mut buf_reader = BufReader::new(file);
        let mut rdr = csv::Reader::from_reader(buf_reader);
        for result in rdr.deserialize() {
            ret.push(result?);
        }
        Ok(ret)
    }).collect();
}
