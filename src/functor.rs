use core::*;

pub trait Functor: Unplug+Plug<<Self as Unplug>::A> {
    fn map<B, F>(f:F, s:Self) -> <Self as Plug<B>>::result_t where
        Self:Plug<B>,
        F:Fn(<Self as Unplug>::A) -> B
        ;
}

impl<A> Functor for Concrete<Box<forall_t>,A> {
    fn map<B,F>(f:F, s:Self) -> <Self as Plug<B>>::result_t where
        F:Fn(<Self as Unplug>::A) -> B
    { 
        Concrete::of(Box::new(f(*(s.unwrap))))
    }
}

impl<A> Functor for Concrete<Vec<forall_t>,A> {
    fn map<B,F>(f:F, s:Self) -> <Self as Plug<B>>::result_t where
        F:Fn(<Self as Unplug>::A) -> B
    { 
        Concrete::of(s.unwrap.into_iter().map(f).collect())
    }
}

impl<A> Functor for Concrete<Option<forall_t>,A> {
    fn map<B,F>(f:F, s:Self) -> <Self as Plug<B>>::result_t where
        F:Fn(<Self as Unplug>::A) -> B
    {
        Concrete::of(s.unwrap.map(f))
    }
}

#[cfg(test)]
fn it_compiles<F:Functor,A,B,C>(functor:F, fun:impl Fn(A)->B, fun2:impl Fn(B)->C) -> <F as Plug<C>>::result_t where
    F:Plug<A>+Plug<B>+Plug<C>+Unplug<A=A>,
    <F as Unplug>::F: Plug<A> + Plug<B> + Plug<C>
{
    let cmp = |x|fun2(fun(x));
    Functor::map(cmp, functor)
}
