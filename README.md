# wasm-test-import

This tests whether is is viable to have an imported function with the same name as a function
that is exported from an rlib which are both have the same name. In this case "call".
It turns out that this is possible if at least one of them has name mangeling enabled.

```sh
cargo build --release --target wasm32-unknown-unknown
wasm2wat target/wasm32-unknown-unknown/release/contract.wasm
```

Yields the following module (some unrelated stuff was removed from the listing):

```wat
(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (import "seal0" "call" (func $_ZN8contract4seal4call17h598b41fa1bda15e7E (type 0)))
  (func $start (type 1)
    call $_ZN8contract4seal4call17h598b41fa1bda15e7E
    drop
    call $call
    drop)
  (func $call (type 0) (result i32)
    i32.const 7)
  (export "start" (func $start))
  (export "call" (func $call))
 ```

 We can see that the viability of this approach is based on the fact that we have name
 mangling for the imported function:

```rust
mod seal {
    #[link(wasm_import_module = "seal0")]
    extern "C" {
        // Enable the no mangle attribute and this clashes with the "call" function from
        // dep which results in this function never imported nor called. The result is that
        // the "call" from dep is called two times.
        // #[no_mangle]
        pub fn call() -> u32;
    }
}
```

Disabling mangling for the import butchers the module in that it calls the function from
the "dep" dependy two times.

```wat
(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (func $start (type 0)
    call $call
    drop
    call $call
    drop)
  (func $call (type 1) (result i32)
    i32.const 7)
  (export "start" (func $start))
  (export "call" (func $call))
```

That said, there is no apparent reason to use `#[no_mangle]` for the imported function. For
the dependency this was merely to emulate a C library.

Writing contracts in languages without name mangling should not be a problem, either.
C compilers support the [import_name attribute](https://clang.llvm.org/docs/AttributeReference.html#import-name).
Even if the embedder uses a non-prefixed imported function like `call` we can avoid
clashed in C like this

```C
// Add a prefix on import to avoid possible name clashes
uint32_t imported_call() __attribute__((import_module(<seal0>, import_name(<call>))));
```
