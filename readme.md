# Type Plugs - A Method for emulating Higher-Kinded Types in Rust

This is a small demonstration of a technique for emulating Higher-Kinded Types (HKTs)/Generalised Associated Types(GATs) through existing language features.
Basically, you can emulate `T::F<X>` as being `<T::F as Plug<X>>::result_t` with the `Plug` trait, and you can destructure `F<X>` down to `F<_>, A` through the `Unplug` trait. 


You can read more about how this works [with the original post](https://gist.github.com/edmundsmith/855fcf0cb35dd467c29a9350481f0ecf) and [the followup](https://gist.github.com/edmundsmith/e09d5f473172066c0023ef84ee830cad).

