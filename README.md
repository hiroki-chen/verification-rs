# Playground for different verification tools for Rust

This repo was created to showcase the differences and common aspects of current state-of-the-art verification tools for the Rust programming language. We consider three tools:

* Kani: <https://github.com/model-checking/kani>
* Prusti <https://github.com/viperproject/prusti-dev>
* Creusot: <https://github.com/xldenis/creusot>

## Detailed designs of these tools

* Kani: It is based on the C-Bounded Model Checker (CBMC) by converting the MIR into C-like `goto`-statements. It also extracts information about the Rust trait to gain additional function pointer restrictions for the generated C code to reduce the verification time. Kani is more like a symbolic execution tool that will generate arbitrary input to test if the code works as expected. Thus, it cannot work on generic types and can only work on a concrete instance. This is due to the fact that model checker is based on a **finite-state** of a program. It thus cannot reason about arbitrary code. For example, consider the following code:

```rust
pub fn sort<T: PartialOrd>(vec: &mut Vec<T>) {
    // implementation
}

pub fn is_sorted<T: PartialOrd>(vec: &Vec<T>) -> bool {
    // implementation
}

#[cfg(kani)]
#[kani::proof]
fn verify() {
    const X: usize = 10;
    let mut vec = kani::any::<[u64; X]>().to_vec();
    sort(&mut vec);
    assert!(is_sorted(&vec));
}
```

First, it only works on a concrete instance of `vec<T>`, and in the above example, we specify the type to be `u64`. Second, the length of the vector will **significantly** affect the verification time because kani would generate *all* possible values that fill the vector to be sorted, and the time will explode when `X` is large. This is because there are $2^{64}$ possible values for each element in the vector!

* Prusti: This is a deductive verification tool based on an intermediate verification language called *Viper* that reasons about the memory model of the Rust programming language instead of soly relying on the compiler to enforce the ownership rule. The SMT solver it uses is Z3. However, this tool is still under development and not very usable, especially for complex projects that heavily use the `std` library because Prusti is awkward at dealing with generics, traits, and other cool language features of Rust. For example, it cannot verify `Vec<T>` due to trait constraints:

  ```rust
  impl<T, I, A: Allocator> Vec<T> where I: SliceIndex<[T]> {
      // Some functions.
  }
  ```

  We must give such functions **specifications** and mark them as `#[trusted]` or `#[pure]` to allow Prusti to know what these functions are doing. While it might not appear harmful when we only want to verify a simple program that encompasses only a small subset of Rust features, the efforts being put on the trivia incurred by Prusti might be meaningless. In addition, Prusti has problems dealing with `for` loops and can only analyze very simple `while` loops.

  The idea of deductive verification, nevertheless, is somehow more powerful when we simply want to verify the functionality of an algorithm since reasoning is performed on an abstract version of the original program, and we do not need to worry about the size explosion incurred by generics and traits. The idea of an intermediate verification language is also very interesting. We hope that the Prusti frontend could become more robust and mature.

* **Creusot:** Creusot is also a deductive verification tool for Rust, and its annotations are akin to those of Prusti. The working mechanism, however, is much more different. The tool lowers Rust's MIR into WhyML, a variant of the ML dialect of the Why3 proof environment. Then WhyML is loaded into Why3 where the proofs are generated and sent to SMT solvers like Alt-Ergo, CVC, or Z3.

  Good news is that Creusot seems to be more usable and promising than Prusti. An SAT solver written in Rust has been *formally verified* using Creusot (see [CreuSAT](https://github.com/sarsko/CreuSAT)). There are some language correspondence between  Prusti and Creusot:

  * `#[requires]` and `#[requires]`.
  * `#[ensures]` and `#[ensures]`.
  * `#[pure]` and `#[predicate]` or `#[logic]`.
  * `#[trusted]` and `#[trusted]`.
  * `predicate!` and `pearlite!`.
  * `#[trusted]` and `#[trusted]`.
  * `body_invariant!` and `#[invariant]`.
  * `old` and `ghost!` (to get the snapshot of a mutable variable).
  * Creusot's Rust specific logical expressions: access to the final value of a mutable reference `^`, access to the model of an object `@`.

## References

* <https://grk2767.tu-dresden.de/files/Images/people/chair-cc/theses/2303_Hayess_MA.pdf>
* <https://dl.acm.org/doi/pdf/10.1145/3510457.3513031>
* <https://dl.acm.org/doi/abs/10.1145/3427761.3432348>
