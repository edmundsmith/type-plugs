use core::*;
use applicative::*;

pub trait Monad : Applicative {
    fn bind<F,B>(f:&mut F, s:Self) -> <Self as Plug<B>>::result_t
    where 
        Self:Plug<F>+Plug<B>,
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone
    ;
}

impl<A> Monad for Box<A> {
    fn bind<F,B>(f:&mut F, s:Self) -> <Self as Plug<B>>::result_t
    where 
        Self:Plug<F>+Plug<B>,
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t
    {
        f(*s)
    }
}

impl<A:Clone> Monad for Vec<A> {
    fn bind<F,B>(f:&mut F, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone
    {
        let res:Vec<B> = 
            s
            .into_iter()
            .map(|x|f.clone()(x))
            .flatten().collect();
        res
    }
}

impl<A> Monad for Option<A> {
    fn bind<F,B>(f:&mut F, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t
    {
        match s {
            Some(x) => f(x),
            None => None
        }
    }
}
