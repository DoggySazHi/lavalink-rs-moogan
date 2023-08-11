# lavalink-rs

A `lavalink` API wrapping library for every `tokio` discord crate.

## To-Do

- [ ] `native_tls` backend
- [ ] Player queues
- [ ] Documentation
- [ ] Expand event logging
- [ ] Examples
- [ ] Songbird utilities
- [ ] Serenity utilities
- [ ] Twilight utilities
- [ ] Lavasnek (PyO3)
- [ ] Tests
- [ ] Implement abstractions for ease of use
- [ ] Implement more node selection methods:
 - [ ] Round-Robin
 - [ ] Region based
 - [x] Load balancer
 - [ ] Main and fallback



## Links to download stuff you will need

- [Lavalink repository](https://github.com/lavalink-devs/Lavalink) (V4)
- [Java download](https://adoptium.net/) (JRE 17)

## How to use

Install the version from crates.io:

```toml
lavalink-rs = "0.10.0-alpha"

# or

[dependencies.lavalink-rs]
version = "0.10.0-alpha"
```

Or the development release:

```toml
lavalink-rs = { git = "https://gitlab.com/vicky5124/lavalink-rs/", branch = "main"}

# or

[dependencies.lavalink-rs]
git = "https://gitlab.com/vicky5124/lavalink-rs/"
branch = "main"
```

If you wish to use a development version of songbird, add the following to the Cargo.toml:

```toml
[patch.crates-io.serenity]
git = "https://github.com/serenity-rs/songbird"
branch = "next"
```

### Features

None for now.
