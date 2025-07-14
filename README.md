# Wave-Function-Collapse

This project is a simple example of the Wave Function Collapse algorithm, which is used for procedural content
generation. The algorithm is based on the concept of superposition in quantum mechanics.

This implementation uses a matrix (2D array of integers) of a size defined by the user and propagates the
values based on a ruleset that is provided via a YAML file.

## Table of Contents

- [Local Setup](#local-setup)
- [CLI](#cli)
- [Rules Definition](#rules-definition)
- [Displaying](#displaying)

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

## CLI

You can optionally pass command line arguments to the program to specify the size of the matrix and the path to the
rules file. If you don't specify the number of columns and rows, the program will prompt you to enter the size of the
matrix. Further you can specify the path to the rules file, if you don't specify it, the program will look for a file
named `rules.yaml`.

To see more information about the command line arguments, you can run the program with the `--help` flag:

## Rules Definition

The rules are defined in a YAML file located in the root directory of the project (or at the same level as the built
binary) and must be named `rules.yaml`. The rules are defined as a list of fields, where each field represents a type of
tile/state within the matrix. Each field has a map of allowed neighbors in each direction (UP, RIGHT, DOWN, LEFT) with
their corresponding weights. The weights are used to determine the probability of each neighbor being selected.
Although the weights can be an arbitrary float value, it is recommended to use values that sum up to 1.0 for each
direction. The weights must be positive values that sum to more than 0.0 per direction.

> [!NOTE]  
> You may not use the value `-1` in the rules, as it is reserved for the empty/superposition state of the matrix.

Example:

```

rules:

# Rule for field 1

- field: 1
  # Allowed neighbors in the UP direction with their respective weights
  allowed_up:
  1: 0.33 # Field 1 has a 33% chance of being above
  2: 0.67 # Field 2 has a 67% chance of being above
  # Allowed neighbors in the RIGHT direction
  allowed_right:
  1: 0.5 # Field 1 can always be to the right with 50% chance
  3: 0.5 # Field 3 can always be to the right with 50% chance
  # Allowed neighbors in the DOWN direction
  allowed_down:
  2: 1.0 # Field 2 will always be below field 1
  # Allowed neighbors in the LEFT direction
  allowed_left:
  1: 0.4 # Field 1 has a 40% chance of being to the left
  3: 0.6 # Field 3 has a 60% chance of being to the left

# Rule for field 2

- field: 2
  ...

```

> [!WARNING]  
> Note that inconsistent rules will cause the algorithm to fail. For example, if a field evaluates to have no allowed
> states it could collapse into.

## Displaying

If the matrix is too large (above 100x100) the program will not display the matrix in the terminal.
For smaller matrices, the program will display the matrix as a colored grid (For now with hardcoded colors & values).
If you set the constant PRINT_COLORFUL in the file src/main.rs to false, the program will display the matrix as a simple
grid of numbers.

You can specify custom colors for each field within the rules file, e.g.

```

rules:

- field: 1
  color: red
  ...
- field: 2
  color: blue

```

The colors are optional and if they are not specified, the program will use the default fallback colors.
Allowed value for the `color` field are:

- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `purple`
- `cyan`
- `white`
- `bright black`
- `bright red`
- `bright green`
- `bright yellow`
- `bright blue`
- `bright magenta`
- `bright cyan`
- `bright white`
