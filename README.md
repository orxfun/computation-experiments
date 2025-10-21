# computation-experiments

This repo is a place where we share, discuss and run different experiments on computations, mainly using rust 🦀

We can create a new folder for each case and work on them in parallel.

## Cases

### Parallel Computation over Non-Linear Data Structures

*You may find all examples and experiments [here](https://github.com/orxfun/computation-experiments/tree/main/src/parallelization-over-nonlinear-data)*.

We can conveniently perform parallel computation over linear data structures such as slices. It is more difficult to do it with non-linear structures such as trees or directed acyclic graphs.

One way to achieve this is to use **rayon**'s scoped computations. Scopes allow to define the lifetime relations so that we do not get lifetime errors. Inside the scoped block, we can define our computation. This is flexible since we can define custom computations. Further, since the lifetime relations are handled with scopes, we can recursively call the function. Please see [`in_place_scope`](https://docs.rs/rayon/1.11.0/rayon/fn.in_place_scope.html) for details.

It is also possible with the new version of **orx-parallel** which takes a different approach. It defines such computations as regular parallel iterators; i.e., we continue to define computations as a chain of transformations over an iterator. This means we have access to entire `ParIter` api. It does this by defining the input part of the computation with two components: initial elements and an `extend` method. Please see [`IntoParIterRec`](https://docs.rs/orx-parallel/latest/orx_parallel/trait.IntoParIterRec.html) for details; or [`into_par_rec_iter`](https://github.com/orxfun/orx-parallel/blob/special-termination-condition/src/iter/recursive/into_par_rec_iter.rs) if not merged yet.

```bash
cd src/parallelization-over-nonlinear-data/

cargo run --release

cargo run --release -- --amount-of-work 10
```

### Length Delimited Codec

*You may find all examples and experiments [here](https://github.com/orxfun/computation-experiments/tree/main/src/length-delimited-codec)*.
