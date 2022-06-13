# CSV Parsing Benchmark

The goal of this benchmark is to understand and evaluate the performance of parsing CSV using the [csv](https://docs.rs/csv/latest/csv/) crate.

The code for this is in [parse.rs](./benches/parse.rs). The benchmark is run on the [kaggle stock data](https://www.kaggle.com/datasets/paultimothymooney/stock-market-data). 

This data is assumed to be placed in '../../datasets/kaggle_stock_data/data/' relative to this folder.

We first establish a few baseline metrics:
- Reading each file to memory sequentially.
- Reading each file to memory in parallel (using rayon).
- Iterating through every character after reading each file in parallel.

Then we bench mark the following:

- Iterate as a `csv::StringRecord` making a new copy for each record
- Iterate as a `csv::ByteRecord` making a new copy for each record
- Iterate as a `csv::ByteRecord` with a single `ByteRecord` copy (effectively minimizing memory allocations per record).

## Results

### MacBook (Intel(R) Core(TM) i7-8850H CPU @ 2.60GHz)

|benchmark|estimate (ms) |lower (ms)|upper (ms)|
|---------|--------|-----|-----|
|kaggle_stock_data_benchmark/seq_io_baseline|429.37|382.61|485.34|
|kaggle_stock_data_benchmark/par_io_baseline|274.27|265.4|283.2|
|kaggle_stock_data_benchmark/par_baseline_each_char|277.7|267.94|287.96|
|kaggle_stock_data_benchmark/csv_file_reader_string|3247.19|2816.11|3735.19|
|kaggle_stock_data_benchmark/csv_file_reader_byte|3796.21|3558.09|4033.9|
|kaggle_stock_data_benchmark/csv_file_reader_record_reference|1286.32|1253.44|1318.16|

### Intel(R) Core(TM) i5-10600K CPU @ 4.10GHz

|benchmark|estimate (ms) |lower (ms)|upper (ms)|
|---------|--------|-----|-----|
|kaggle_stock_data_benchmark/seq_io_baseline|180.19|180.08|180.32|
|kaggle_stock_data_benchmark/par_io_baseline|105.86|105.82|105.91|
|kaggle_stock_data_benchmark/par_baseline_each_char|105.75|105.55|105.88|
|kaggle_stock_data_benchmark/csv_file_reader_string|565.96|565.65|566.32|
|kaggle_stock_data_benchmark/csv_file_reader_byte|513.13|512.65|513.69|
|kaggle_stock_data_benchmark/csv_file_reader_record_reference|364.42|363.78|365.62|
## Conclusions

- StringRecord vs ByteRecord seems to make a marginal difference in this dataset.
- Reusing a ByteRecord makes a huge difference.

## Follow Up Questions

- Does the serde conversion path in the csv crate use a single ByteRecord?
    - Yes
- arrow2 uses a single ByteRecord for inferring schema, but uses a batch of ByteRecords for deserializing into arrow. Can it benefit from a single ByteRecord?
- How does arrow2_convert performance compare, where we use a statically typed schema, and use serde to convert to an intermediate struct?
- There are some complex match expressions in the csv library, and from prior experience these seem to cause a performance hit. Look into using a lookup table for optimization.
    - After further reading this seems to be optimization performed by LLVM:
        - [Example of a neat LLVM optimization](https://www.reddit.com/r/rust/comments/31kras/are_match_statements_constanttime_operations/)