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
# 
./flatc.exe --rust -o src/fbs/notification_v0/ src/fbs/notification_v0/notification_v0.fbs
# 
./flatc.exe --rust -o src/fbs/status_v0/ src/fbs/status_v0/status_v0.fbs
# 
./flatc.exe --rust -o src/fbs/trigger_v0 src/fbs/trigger_v0/trigger_v0.fbs
# 
./flatc.exe --rust -o src/fbs/vector_f32_v0 src/fbs/vector_f32_v0/vector_f32_v0.fbs
```



