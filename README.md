# DBSCAN

> Density-Based Spatial Clustering of Applications with Noise

[Wikipedia link](1)

DBSCAN is a density-based clustering algorithm: given a set of points in some space, it groups together points that are closely packed together (points with many nearby neighbors), marking as outliers points that lie alone in low-density regions (whose nearest neighbors are too far away).

This is an implementation of the algorithm in rust-stable.

## Usage

### Prerequisites

This project is written entirely in rust. It is recommended that you use the latest stable version with it.

Oldest supported version: `1.26.1`

### Installation

Add the project to your `Cargo.toml` file, under dependencies. At the moment, there are no optional features, so this will suffice:

**Cargo.toml**

```toml
[dependencies]
dbscan = "0.1"
```

Import the library into your project using:

```rust
extern crate dbscan;
use dbscan::{DBSCAN, HasDistance};
```

This will add the trait `HasDistance` and the struct `DBSCAN` to the current module.

### Implementation example

Implementation examples are provided in the `examples/` directory. One simple implementation is presented below.

**2D Point clustering**
This implementation uses the `dbscan` crate to add distance-based clustering capabilities to a field of 2D points.

```rust
// TODO: Example here
```

## Development

Setting up a development environment can be done as follows.

### Setup

1.  Install the latest stable version of rust if you haven't done so already.
2.  Clone this repository into your dev directory

It is recommended that you use use `rustfmt` to keep consistent formatting while working on the project. Most IDEs and editors should support it natively.

### Tests

TODO

## Contributing

Please read CONTRIBUTING.md for the process of submitting pull requests.

## Versioning

This project uses SemVer for versioning. For the versions available, see the tags on this repository.

## Authors

_Primary:_ Alan K <mailto:afksavish@gmail.com> @savish

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

[1]: https://en.wikipedia.org/wiki/DBSCAN
