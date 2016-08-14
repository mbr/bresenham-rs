bresenham-rs
============

Implements [Bresenham's line drawing algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm) in Rust using an iterator over all points in the line. Most, if not all overhead should evaporate when inline by the compiler.

Example use:

```rust
for (x, y) in Bresenham::new((0, 1), (6, 4)) {
    println!("{}, {}", x, y);
}

```

Will print:

```
(0, 1)
(1, 1)
(2, 2)
(3, 2)
(4, 3)
(5, 3)
```
