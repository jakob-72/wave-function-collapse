# Wave-Function-Collapse

This project is a simple example of the Wave Function Collapse algorithm, which is used for procedural content
generation. The algorithm is based on the concept of superposition in quantum mechanics.

This implementation uses a matrix (2D array of positive integers) of a size defined by the user and propagates the
values
based on a ruleset that is provided via a YAML file.

## Local Setup

This project requires Rust to be installed. You can install Rust by following the instructions
at https://www.rust-lang.org/tools/install.

To run the project execute the command:

```
cargo run
```

To build an optimized version, use the command:

```
cargo build --release
```

The optimized binary will be located in the `target/release` directory. It will be much faster & performant than the
debug version.

## Rules Definition

The rules are defined in a YAML file located in the root directory of the project (or at the same level as the built
binary) and must be named `rules.yaml`. The rules are defined as a list of fields, where each field represents a type of
tile/state within the matrix. Each field has a set of allowed neighbors, which are defined as a list of integers.

Example:

```
rules:
  - field: 1 # water
    allowed_up: [1, 2]      # water can have water or beach above it
    allowed_right: [1, 2]   # water can have water or beach to its right
    allowed_down: [1, 2]    # water can have water or beach below it
    allowed_left: [1, 2]    # water can have water or beach to its left
  - field: 2 # beach
    allowed_up: [1, 2, 3]   # beach can have water, beach, or grass above it
    # and so on...
```

Note that inconsistent rules will cause the algorithm to fail. For example, if a field evaluates to have no allowed
states it could collapse into.
