use core::*;
use functor::*;
use std::ops::Deref;

pub trait Applicative: Functor {
    fn pure(s:<Self as Unplug>::A) -> Self;
    fn app<B, F>(f:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t where
        F:Fn(<Self as Unplug>::A) -> B,
        Self:Plug<F>+Plug<B>+Unplug,
        <Self as Plug<F>>::result_t:Unplug<F=<Self as Unplug>::F,A=F>+Plug<F>+Clone,
        <Self as Unplug>::F:Plug<F>
        ;
}

impl<A> Applicative for Box<A> {
    fn pure(a:A) -> Self {
        Box::new(a)
    }

    fn app<B, F>(f:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> B
    {
        Box::new((*f)(*s))
    }
}

impl<A:Clone> Applicative for Vec<A> {
    fn pure(a:A) -> Self {
        vec![a]
    }
    fn app<B, F>(fs:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> B,
        <Self as Plug<F>>::result_t: Clone,
    {
        let flat:Vec<B> = 
        Functor::map(|x:A|
            Functor::map(|f:F|
                f(x.clone()),
            fs.clone()),
        s).into_iter().flatten().collect();
        flat
    }
}

impl<A> Applicative for Option<A> {
    fn pure(a:A) -> Self {
        Some(a)
    }
    fn app<B, F>(fs:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> B
    {
        match fs {
            Some(f) => match s {
                Some(x) => Some(f(x)),
                None => None
            },
            None => None
        }
    }
}