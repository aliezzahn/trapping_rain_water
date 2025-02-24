# Trapping Rain Water

The problem involves calculating the amount of rainwater that can be trapped between the bars of an elevation map represented by a vector of non-negative integers.

## Problem Description

Given an array `height` where each element represents the height of a bar, compute how much water can be trapped after raining.

## Solution

The solution uses a two-pointer approach to efficiently compute the trapped water in `O(n)` time and `O(1)` space.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
trapping_rain_water = { git = "https://github.com/aliezzahn/trapping_rain_water.git" }
```

Then, use the `trap` function in your code:

```rust
use trapping_rain_water::trap;

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("Trapped water: {}", trap(height));
}
```

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```
