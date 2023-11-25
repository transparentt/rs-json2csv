# rs-json2csv

Convert JSON to CSV in Rust.

# Usage

```
[dependencies]
rs_json2csv = {git = "https://github.com/transparentt/rs-json2csv.git"}
```

```rust
use rs_json2csv::json2csv;

fn main() {
    json2csv("input.json", "output.csv")
}
```

# License

MIT
