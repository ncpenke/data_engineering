use criterion::{criterion_group, criterion_main, Criterion};
use csv::ByteRecord;
use glob::glob;
use rayon::prelude::*;
use std::{fs::File, path::PathBuf};
use tokio_util::compat::*;

async fn read_file(f: PathBuf) {
    let file = tokio::fs::File::open(f).await.unwrap().compat();
    let mut reader = csv_async::AsyncReaderBuilder::new().create_reader(file);       
    let mut record = csv_async::ByteRecord::new();
    while reader.read_byte_record(&mut record).await.unwrap_or(false) {}
}

async fn process_files(files: Vec<PathBuf>) {
    let promises = files.clone()
        .into_iter()
        .map(read_file)
        .collect::<Vec<_>>();
    futures::future::join_all(promises).await;
}

fn add_benchmark(c: &mut Criterion) {
    let mut files = Vec::<PathBuf>::new();
    glob("./data/*.csv")
        .unwrap()
        .map(|e| e.unwrap())
        .for_each(|e| files.push(e));

    let mut group = c.benchmark_group("parse_baseline");
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

    group.bench_function("par_baseline_iter_char", |b| {
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

    group.bench_function("async_io_reader_reference", |b| {
        b.to_async(tokio::runtime::Builder::new_current_thread().build().unwrap())
            .iter(|| process_files(files.clone()));
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

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
