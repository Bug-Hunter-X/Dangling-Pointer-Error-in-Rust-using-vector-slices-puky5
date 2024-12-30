This repository demonstrates a common error in Rust programming: creating dangling pointers when working with vectors and slices. The `bug.rs` file contains code that produces a dangling pointer, leading to undefined behavior. The `bugSolution.rs` file provides a corrected version that demonstrates how to avoid this issue by ensuring the vector outlives the slice.