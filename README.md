# primitive-buffer

A stack-based buffer with fixed capacity for primitives.

Uses uninitialized memory to avoid the cost of filling the buffer with default values.

## Examples

```rust
use primitive_buffer::Buffer;

let mut buffer: Buffer<u8, 8> = Buffer::new();

buffer.push(1);
buffer.push(2);

assert_eq!(buffer.len(), 2);

assert_eq!(buffer.pop(), Some(2));
assert_eq!(buffer.len(), 1);

buffer.clear();

assert!(buffer.is_empty());
```
