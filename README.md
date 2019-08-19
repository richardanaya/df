# df
An ergonomic DataFrames library for Rust inspired by data science libraries like (pandas)[https://pandas.pydata.org/] and tools like (jupyter)[https://jupyter.org/] and (evcxr)[https://github.com/google/evcxr]. `df` hopes to make Rust's performance accessable in daily tools used in numerical analysis.

```rust
let dataframe = DataFrame::new(
    vec!["width", "height", "name", "in_stock", "count"],
    data![
            0.4, 0.7, "book", true, 1, 
            3.0, 4.7, "poster", true, 1
        ],
);
```

# Get Started

1) Setup jupyter and evcxr using (these instructions)[https://github.com/google/evcxr/blob/master/evcxr_jupyter/README.md]
2) Start jupyter `jupyter notebook`
3) Create a notebook with language `Rust`
