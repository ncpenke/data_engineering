use criterion::{criterion_group, criterion_main, Criterion};
use csv::ByteRecord;
use glob::glob;
use rayon::prelude::*;
use std::fs::File;

const EXCHANGES: &'static [&'static str] = &[
    "nyse",
    "forbes2000",
    "nyse",
    "sp500"
];

fn add_kaggle_stock_data_benchmark(c: &mut Criterion) {
    let mut files: Vec<std::path::PathBuf> = vec![];
    EXCHANGES.iter().for_each(|e| {
        glob(&format!("../../datasets/kaggle_stock_data/data/{}/csv/*.csv", e))
            .unwrap()
            .map(|e| e.unwrap())
            .for_each(|e| files.push(e));
    });

    let mut group = c.benchmark_group("kaggle_stock_data_benchmark");
    group.sample_size(10);

    group.bench_function("seq_io_baseline", |b| {
        b.iter(|| {
            let _ = files.iter().for_each(|f| {
                let _ = std::fs::read(f).unwrap();
            });
        });
    });

    group.bench_function("par_io_baseline", |b| {
        b.iter(|| {
            let _ = files.par_iter().for_each(|f| {
                let _ = std::fs::read(f).unwrap();
            });
        });
    });

    group.bench_function("par_baseline_each_char", |b| {
        b.iter(|| {
            let _ = files.par_iter().for_each(|f| {
                let s = std::fs::read(f).unwrap();
                for _ in s {}
            });
        });
    });

    group.bench_function("csv_file_reader_string", |b| {
        b.iter(|| {
            let _ = files.par_iter().for_each(|f| {
                let f = File::open(f).unwrap();
                let rdr = csv::Reader::from_reader(f);
                for result in rdr.into_records() { 
                    let _ = result.unwrap(); 
                }
            });
        });
    });

    group.bench_function("csv_file_reader_byte", |b| {
        b.iter(|| {
            let _ = files.par_iter().for_each(|f| {
                let f = File::open(f).unwrap();
                let rdr = csv::Reader::from_reader(f);
                for result in rdr.into_byte_records() { 
                    let _ = result.unwrap(); 
                }
            });
        });
    });

    // Note: doesn't seem to make a difference so commented out
    /*
    group.bench_function("csv_byte_reader_byte", |b| {
        b.iter(|| {
            let _ = files.par_iter().for_each(|f| {
                let bytes = std::fs::read(f).unwrap();
                let rdr = csv::Reader::from_reader(&*bytes);
                for result in rdr.into_byte_records() { 
                    let _ = result.unwrap(); 
                }
            });
        });
    });
    */

    group.bench_function("csv_file_reader_record_reference", |b| {
        b.iter(|| {
            let _ = files.par_iter().for_each(|f| {
                let f = File::open(f).unwrap();
                let mut rdr = csv::Reader::from_reader(f);
                let mut record = ByteRecord::new();
                while rdr.read_byte_record(&mut record).unwrap_or(false) {}
            });
        });
    });

}

criterion_group!(benches, add_kaggle_stock_data_benchmark);
criterion_main!(benches);
