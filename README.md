# Advent Of Code Cli

This library is a helper that will handle the logic behind running a day 
when doing the **Advent of code**.

## Installation

```sh
cargo add --git https://github.com/TGuillouet/aoc_cli_rust.git 
```

## Setup

In your main.rs
```rust
fn main() {
    let mut days: HashMap<u32, Box<dyn RunnableDay>> = HashMap::new();
    days.insert(1, Box::new(Day01::default())); // Check the examples to see how to implement a day

    let _ = aoc_cli::init(days);
}
```

## Usage

```sh
# Run the current day
cargo run

# Run a specific day of a specific year
cargo run -- -d 1 -y 2016
```