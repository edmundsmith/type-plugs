use core::*;
use functor::*;
use std::ops::Deref;

pub trait Applicative: Functor {
    fn pure(s:<Self as Unplug>::A) -> Self;
    fn app<B, F>(f:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t where
        F:Fn(<Self as Unplug>::A) -> B+Clone,
        Self:Plug<F>+Plug<B>+Unplug,
        <Self as Plug<F>>::result_t:Unplug<F=<Self as Unplug>::F,A=F>+Plug<F>+Clone,
        <Self as Unplug>::F:Plug<F>
        ;
}

impl<A> Applicative for Concrete<Box<forall_t>,A> {
    fn pure(a:A) -> Self {
        Concrete::of(Box::new(a))
    }

    fn app<B, F>(f:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> B
    {
        Concrete::of(
            Box::new((*f.unwrap)(*s.unwrap))
        )
    }
}

impl<A:Clone> Applicative for Concrete<Vec<forall_t>,A> {
    fn pure(a:A) -> Self {
        Concrete::of(vec![a])
    }
    fn app<B, F>(fs:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> B + Clone,
        <Self as Plug<F>>::result_t: Clone,
    {
        let flat:Vec<B> = 
        Functor::map(|x:A|
            Functor::map(|f:F|
                f.clone()(x.clone()),
            fs.clone()),
        s).unwrap.into_iter().map(|x|x.unwrap).flatten().collect();
        Concrete::of(flat)
    }
}

impl<A> Applicative for Concrete<Option<forall_t>,A> {
    fn pure(a:A) -> Self {
        Concrete::of(Some(a))
    }
    fn app<B, F>(fs:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> B
    {
        Concrete::of(
            match fs.unwrap {
                Some(f) => match s.unwrap {
                    Some(x) => Some(f(x)),
                    None => None
                },
                None => None
            }
        )
    }
}