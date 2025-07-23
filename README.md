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

## Credentials and roles


|       | writer_certificate.pem                   | logger_certificate.pem    | default_certificate.pem               | bad_client_certificate.pem | expired_client_certificate.pem |
| ------- | ------------------------------------------ | --------------------------- | --------------------------------------- | ---------------------------- | -------------------------------- |
| role  | writer                                   | logger/reader             | no role                               | no role                    | no role                        |
| usage | able to pub/sub except on platform topic | able to sub to all topics | connect to platform but can't pub/sub | can't connect to platform  | can't connect to platform      |
