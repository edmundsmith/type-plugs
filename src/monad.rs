use core::*;
use applicative::*;

pub trait Monad : Applicative {
    fn bind<F,B>(f:&mut F, s:Self) -> <Self as Plug<B>>::result_t
    where 
        Self:Plug<F>+Plug<B>,
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone
    ;
}

impl<A> Monad for Concrete<Box<forall_t>,A> {
    fn bind<F,B>(f:&mut F, s:Self) -> <Self as Plug<B>>::result_t
    where 
        Self:Plug<F>+Plug<B>,
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t
    {
        f(*s.unwrap)
    }
}

impl<A:Clone> Monad for Concrete<Vec<forall_t>,A> {
    fn bind<F,B>(f:&mut F, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone
    {
        let res:Vec<B> = 
            s.unwrap
            .into_iter()
            .map(|x|f.clone()(x).unwrap)
            .flatten().collect();
        Concrete::of(res)
    }
}

impl<A> Monad for Concrete<Option<forall_t>,A> {
    fn bind<F,B>(f:&mut F, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t
    {
        match s.unwrap {
            Some(x) => f(x),
            None => Concrete::of(None as Option<B>)
        }
    }
}
