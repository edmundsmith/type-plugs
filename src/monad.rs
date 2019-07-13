use core::*;
use applicative::*;

pub trait Monad : Applicative {
    fn bind<F,B>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where 
        Self:Plug<F>+Plug<B>,
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t
    ;
}

impl<A> Monad for Box<A> {
    fn bind<F,B>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where 
        Self:Plug<F>+Plug<B>,
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t
    {
        f(*s)
    }
}

impl<A:Clone> Monad for Vec<A> {
    fn bind<F,B>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t
    {
        let res:Vec<B> = 
            s
            .into_iter()
            .map(|x|f(x))
            .flatten().collect();
        res
    }
}

impl<A> Monad for Option<A> {
    fn bind<F,B>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:Fn(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t
    {
        match s {
            Some(x) => f(x),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn higher_poly_demo<'a,M:Monad,A:'a+Clone,B:'a+Clone,F>(m:M, f:F) -> <M as Plug<B>>::result_t where
        M:Plug<A>+Plug<B>+Unplug<A=A>,//+Plug<F>+Plug<Fn(A)-><M as Plug<B>>::result_t>,
        M:Plug<Box<Fn(A)-><M as Plug<B>>::result_t>>,
        M:Plug<F>,
        F:'static,
        <M as Unplug>::F:Plug<A>+Plug<B>,
        <M as Plug<B>>::result_t:Monad+Unplug<A=B>+'a,
        <<M as Plug<B>>::result_t as Unplug>::F:Plug<B>,
        F:Fn(A) -> B+'a,
        //F:Fn(A) -> <M as Plug<B>>::result_t + Clone,
    {
        let cl = Box::new(move |x|Applicative::pure(f(x)));
        Monad::bind::<Box<Fn(A)->_>,B>(cl as Box<Fn(A)->_>, m)
    }

    #[test]
    fn use_higher_poly() {
        let f = |x|x+1;
        let p1 = Some(5);
        let p2 = vec![5];
        let p3 = Box::new(5);
        assert!(higher_poly_demo(p1, f) == Some(6));
        assert!(higher_poly_demo(p2, f) == vec![6]);
        assert!(higher_poly_demo(p3, f) == Box::new(6));
    }
}