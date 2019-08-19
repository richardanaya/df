# df
dataframes for rust

```rust
let dataframe = DataFrame::new(
    columns!["width","height","name"],
    data![
            .4,.7,"book",
            3,4,"poster"
        ]
)?;
```