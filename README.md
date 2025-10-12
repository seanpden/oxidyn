# Oxidyn

A minimal systems dynamic modeling library for Rust, inspired by STELLA.

## About the Project

Oxidyn is a lightweight systems dynamics library that enables modeling complex dynamic systems through stocks, flows, and their connections. It is designed for researchers, educators, and engineers who need to simulate feedback systems, understand system behavior over time, and explore how different components interact.

### Built With

- Rust

Oxidyn is built in pure Rust and has minimal dependencies.

## Getting Started

Oxidyn is designed to be as minimal as possible, meaning that it should be easy to integrate this library into any project.

### Prerequisites

- Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or visit [rust-lang](https://rust-lang.org/tools/install/) for other installation methods.

### Installation

Add Oxidyn to your `Cargo.toml`:

```toml
[dependencies]
oxidyn = { git = "https://github.com/seanpden/oxidyn"}
```

### Usage

```rust
fn main() {
  println!("Hello world!")
}
```

### Notes

This library was initially developed to support my research into modeling cognition as a dynamic system. After some consideration, I've decided to make the library a bit more generalized. Oxidyn is in early development, current features align with my initial research goal and, as such, the API might significantly change.

## Roadmap

- [x] Basic Stocks
- [x] Basic Flows
- [x] Model, Simulation Loop, System State
- [ ] Basic Converters
- [ ] Basic Connectors
- [ ] Model export
- [ ] Simulation results export
- [ ] Converters

*Any major feature plans here. Optionally link to a dedicated document for features.*

## Contributing

Contributions are what make the open source community such an amazing place to
learn, inspire, and create. Any contributions you make are greatly appreciated.

If you have a suggestion that would make this better, please fork the repo and
create a pull request. You can also simply open an issue with the tag
"enhancement". Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (git checkout -b feature/amazing-feature)
3. Commit your Changes (git commit -m 'Add some amazing-feature')
4. Push to the Branch (git push origin feature/amazing-feature)
5. Open a Pull Request

## License

Distributed under the Apache 2.0 license. See `LICENSE` for more information

## Contact

All communication should be done through the `Discussions` page in GitHub.

## Acknowledgments

- This readme was inspired by [this template](https://github.com/othneildrew/Best-README-Template/blob/main/README.md)
- Heavily inspired by the functionality of [STELLA](https://iseesystems.com/)
<!-- - [Static badge](https://shields.io/badges) used for the "Built With" badges -->
