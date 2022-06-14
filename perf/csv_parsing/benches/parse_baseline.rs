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
    let csv_files = glob("./data/*.csv")
        .unwrap()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    
    let json_files = glob("./data/*.json")
        .unwrap()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("parse_baseline");
    group.sample_size(10);

    group.bench_function("csv_seq_io_baseline", |b| {
        b.iter(|| {
            let _ = csv_files.iter().for_each(|f| {
                let _ = std::fs::read(f).unwrap();
            });
        });
    });

    group.bench_function("csv_par_io_baseline", |b| {
        b.iter(|| {
            let _ = csv_files.par_iter().for_each(|f| {
                let _ = std::fs::read(f).unwrap();
            });
        });
    });

    group.bench_function("json_par_io_baseline", |b| {
        b.iter(|| {
            let _ = json_files.par_iter().for_each(|f| {
                let _ = std::fs::read(f).unwrap();
            });
        });
    });

    group.bench_function("csv_par_baseline_iter_char", |b| {
        b.iter(|| {
            let _ = csv_files.par_iter().for_each(|f| {
                let s = std::fs::read(f).unwrap();
                for _ in s {}
            });
        });
    });

    group.bench_function("json_par_baseline_iter_char", |b| {
        b.iter(|| {
            let _ = json_files.par_iter().for_each(|f| {
                let s = std::fs::read(f).unwrap();
                for _ in s {}
            });
        });
    });

    group.bench_function("csv_file_reader_string", |b| {
        b.iter(|| {
            let _ = csv_files.par_iter().for_each(|f| {
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
            let _ = csv_files.par_iter().for_each(|f| {
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
            .iter(|| process_files(csv_files.clone()));
    });

    group.bench_function("json_deserializer", |b| {
        b.iter(|| {
            let _ = json_files.par_iter().for_each(|f| {
                let data = std::fs::read(f).unwrap();
                let a = json_deserializer::parse(&data).unwrap();
                if let json_deserializer::Value::Array(_) = a {
                } else {
                    panic!()
                }
            });
        });
    });

    group.bench_function("simd_json", |b| {
        b.iter(|| {
            let _ = json_files.par_iter().for_each(|f| {
                let mut data = std::fs::read(f).unwrap();
                use simd_json::Value;
                let a = simd_json::to_borrowed_value(&mut data).unwrap();
                assert!(a.is_array());    
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
            let _ = csv_files.par_iter().for_each(|f| {
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
