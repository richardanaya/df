# df
dataframes for rust

```rust
let dataframe = DataFrame::new(
    vec!["width", "height", "name", "in_stock", "count"],
    data![
            0.4, 0.7, "book", true, 1, 
            3.0, 4.7, "poster", true, 1
        ],
);
```