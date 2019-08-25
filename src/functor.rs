use core::*;

pub trait Functor: Unplug+Plug<unplug!(Self, A)> {
    fn map<B, F>(f:F, s:Self) -> plug!(Self[B]) where
        Self:Plug<B>,
        F:Fn(Self::A) -> B
        ;
}

impl<A> Functor for Box<A> {
    fn map<B,F>(f:F, s:Self) -> plug!(Self[B]) where
        F:Fn(Self::A) -> B
    { 
        Box::new(f(*s))
    }
}

impl<A> Functor for Vec<A> {
    fn map<B,F>(f:F, s:Self) -> plug!(Self[B]) where
        F:Fn(Self::A) -> B
    { 
        s.into_iter().map(f).collect()
    }
}


impl<A> Functor for Option<A> {
    fn map<B,F>(f:F, s:Self) -> plug!(Self[B]) where
        F:Fn(Self::A) -> B
    {
        s.map(f)
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
