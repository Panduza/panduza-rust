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
cargo test --release --test basics

# Run scenario with @focus tag
cargo test --release --test basics_focus
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

| certificate                        | role           | usage                                     |
| ---------------------------------- | -------------- | ----------------------------------------- |
| writer_certificate.pem             | writer         | able to pub/sub except on platform topic  |
| logger_certificate.pem             | logger/reader  | able to sub to all topics                 |
| default_certificate.pem            | no role        | connect to platform but can't pub/sub     |
| bad_client_certificate.pem         | no role        | can't connect to platform                 |
| expired_client_certificate.pem     | no role        | can't connect to platform                 |
