# rslibs

A metarepository containing various libraries I maintain for my typical rust stack.

![Main pipeline](https://github.com/spuxx-dev/rslibs/actions/workflows/main.yml/badge.svg)
![License](https://img.shields.io/github/license/spuxx-dev/rslibs)

## Development

### Coverage

Coverage reports require [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov):

```sh
cargo install cargo-llvm-cov
```

The following aliases are available:

| Command               | Description                                        |
| --------------------- | -------------------------------------------------- |
| `cargo coverage`      | Print a summary table to the terminal              |
| `cargo coverage-html` | Generate an HTML report and open it in the browser |
| `cargo coverage-lcov` | Write an LCOV report to `lcov.info`                |
