use core::{Plug, Unplug, forall_t};

macro_rules! simple_impl {
    ($name:ident) => {
        impl<A,B> Plug<B> for $name<A>{
            type result_t = $name<B>;
        }

        impl<A> Unplug for $name<A> {
            type F = $name<forall_t>;
            type A = A;
        }
    };
}

simple_impl!(Box);
simple_impl!(Vec);
simple_impl!(Option);