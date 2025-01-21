# Rust Mutable Borrow Bug

This repository demonstrates a common error in Rust programming involving mutable and immutable borrows. The `bug.rs` file contains code that attempts to simultaneously hold a mutable and immutable reference to the same variable, resulting in a compile-time error. The `bugSolution.rs` file shows how to resolve this issue.

## How to run

1. Clone the repository: `git clone <repository_url>`
2. Navigate to the directory: `cd <repository_name>`
3. Compile and run the `bug.rs` file using rustc: `rustc bug.rs && ./bug` (this will result in a compile-time error)
4. Compile and run the `bugSolution.rs` file using rustc: `rustc bugSolution.rs && ./bugSolution` (this will execute without errors)
