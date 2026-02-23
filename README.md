# XorShift PRNG in Rust

This repository contains simple implementations of XorShift pseudo-random number generators (PRNGs) written in Rust.

Implemented variants:

- XorShift32 (`u32`)
- XorShift64 (`u64`)
- XorShift128 (`[u32; 4]` state)

These implementations are intended for educational purposes and experimentation with bitwise-based random number generation.

---

## Overview

XorShift generators produce pseudo-random numbers using XOR and bit-shift operations. They are extremely fast and simple but are **not cryptographically secure**.

Each function receives a state struct and returns the next generated value.

---

## Example Code

```rust
struct XorShift32 {
    xor_value: u32
}

fn xor_shift(mut state: XorShift32) -> u32 {
    let mut to_shift = state.xor_value;
    to_shift ^= to_shift << 13;
    to_shift ^= to_shift >> 17;
    to_shift ^= to_shift << 5;

    state.xor_value = to_shift;
    state.xor_value
}

struct XorShift64State {
    to_shift_value: u64
}

fn xor_shift64(mut state: XorShift64State) -> u64 {
    let mut x = state.to_shift_value;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;

    state.to_shift_value = x;
    state.to_shift_value
}

struct XorShift128State {
    x: [u32; 4],
}

fn xor_shift128(mut state: XorShift128State) -> u32 {
    let mut t = state.x[3];
    let s = state.x[0];

    state.x[3] = state.x[2];
    state.x[2] = state.x[1];
    state.x[1] = s;

    t ^= t << 11;
    t ^= t >> 8;

    state.x[0] = t ^ s ^ (s >> 19);
    state.x[0]
}

fn main() {
    let xor32 = XorShift32 { xor_value: 1 };
    let xor_value32 = xor_shift(xor32);
    println!("xor shift value {}", xor_value32);

    let xor64 = XorShift64State { to_shift_value: 1 };
    let xor_value64 = xor_shift64(xor64);
    println!("xor shift value {}", xor_value64);

    let xor128 = XorShift128State { x: [1, 1, 1, 1] };
    let xor_value128 = xor_shift128(xor128);
    println!("the value is {}", xor_value128);
}

```
How to Run

Install Rust (stable version recommended).

Create a new Cargo project or paste the code into main.rs.

Run:
``
cargo run
``


# Important Notes

These generators are not suitable for cryptographic use.

Do not use them for generating tokens, encryption keys, or security-sensitive randomness.

For production applications requiring high-quality randomness, use the rand crate.