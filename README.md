## Introduction

A UB before revm 3.4.0

Reproduce:

```
(default_venv) mio@station /t/ub_repro (master) [0|1]> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/ub_repro`
0x5bf69e2d5c00, aligned: true
0x5bf69e2d5c01, aligned: false
0x5bf69e2d5c02, aligned: false
Continue
(default_venv) mio@station /t/ub_repro (master)> cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/ub_repro`
0x651fc2f01c00, aligned: true
0x651fc2f01c01, aligned: false
0x651fc2f01c02, aligned: false
StackUnderflow
(default_venv) mio@station /t/ub_repro (master)> 
```

Miri:

```
0x873ab, aligned: false
0x873ac, aligned: false
0x873ad, aligned: false
error: Undefined Behavior: `assume` called with `false`
   --> /home/mio/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:239:13
    |
239 |             crate::intrinsics::assume(self < slice.len());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `assume` called with `false`
    |
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: BACKTRACE:
    = note: inside `<usize as std::slice::SliceIndex<[revm_primitives::ruint::Uint<256, 4>]>>::get_unchecked` at /home/mio/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:239:13: 239:58
    = note: inside `revm_primitives::bitvec::macros::internal::core::slice::<impl [revm_primitives::ruint::Uint<256, 4>]>::get_unchecked::<usize>` at /home/mio/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:686:20: 686:45
    = note: inside `revm_interpreter::Stack::pop2_unsafe` at /home/mio/.cargo/git/checkouts/revm-4517530ba533de04/fa764aa/crates/interpreter/src/interpreter/stack.rs:144:14: 144:46
    = note: inside `revm_interpreter::instructions::memory::mstore` at /home/mio/.cargo/git/checkouts/revm-4517530ba533de04/fa764aa/crates/interpreter/src/instructions/macros.rs:132:35: 132:62
note: inside `main`
   --> src/main.rs:37:5
    |
37  |     memory::mstore(&mut intrp, &mut host);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error; 2 warnings emitted
```