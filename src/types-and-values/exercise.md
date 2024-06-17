---
minutes: 15
---

# Exercise: Fibonacci

The Fibonacci sequence begins with `[0,1]`. For n>1, the n'th Fibonacci number
is calculated recursively as the sum of the n-1'th and n-2'th Fibonacci numbers.

Write a function `fib(n)` that calculates the n'th Fibonacci number. When will
this function panic?

```rust,editable,should_panic
{{#include exercise.rs:fib}}
    if n < 2 {
        // The base case.
        todo!("Implement this")
    } else {
        // The recursive case.
        todo!("Implement this")
    }
}

{{#include exercise.rs:main}}
```
## Additional exercices Rustlings

We will be using [`rustlings`](https://github.com/rust-lang/rustlings) for the additional exercises in this course.

Before starting with the exercises, you have to follow the following steps:

- [Install Rust using `rustup`](https://www.rust-lang.org/tools/install)
- [Install VS-Code](https://code.visualstudio.com/) (unless you want to use your own editor with language server support [`rust-analyzer`](https://rust-analyzer.github.io/))
  - [Install the `rust-analyzer` plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- Install Git if not already installed on your system.
- [Follow the instructions in the README](https://github.com/rust-lang/rustlings/blob/main/README.md).
- Open a terminal in VS-Code and run the command `rustlings watch` in it.

If you need any help, you can always ask me 😃
---

You should do the following Rustlings:

- `intro1`
- `intro2`
- `variables1`
- `variables2`
- `variables3`
- `variables4`
- `variables5`


`rustlings watch` will guide you through these exercises in the same order.
But **you should stop after the last exercise listed above**.
The rest of the exercises will be done later.
