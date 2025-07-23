# panduza-rust
Panduza Rust Client

## Tests

First start a platform with this configuration

```json
{
    "devices": [
        {
            "dref": "vi.tester",
            "name": "tester"
        }
    ]
}
```

Then run

```bash
# Run all basics tests
cargo test --test basics

# Run scenario with @focus tag
cargo test --test basics_focus
```

## Flat Buffers

```bash
./flatc.exe --version
# flatc version 25.2.10
```

```bash
# To rebuild flatbuffers
./flatc.exe --rust -o src/fbs/ src/fbs/panduza.fbs
# !!! Then I did some modification to erase warnings
```



