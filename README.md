# axis-ticks

A port of [`d3-ticks`](https://observablehq.com/@d3/d3-ticks), part of the JavaScript D3 plotting library.

Generates an array of nicely rounded values between two numbers which are ideal for positioning axis labels and grid-lines.
```rust
let ticks = axis_ticks::ticks(-0.125, 0.25, 10);

assert_eq!(
    ticks,
    [-0.15, -0.1, -0.05, 0.0, 0.05, 0.1, 0.15, 0.2, 0.25]
);
```

License: MIT
