# df
dataframes for rust

```rust
let dataframe = DataFrame::new(
    columns!["width","height","name"],
    data![
            1,2,"hey",
            4,5,"boo"
        ]
)?;
```