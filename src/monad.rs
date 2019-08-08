use applicative::*;
use core::*;

pub trait Monad: Applicative {
    fn bind<F, B>(f: F, s: Self) -> plug!(Self[B])
    where
        Self: Plug<F> + Plug<B>,
        F: Fn(unplug!(Self, A)) -> plug!(Self[B]);
}

impl<A> Monad for Box<A> {
    fn bind<F, B>(f: F, s: Self) -> plug!(Self[B])
    where
        Self: Plug<F> + Plug<B>,
        F: Fn(unplug!(Self, A)) -> plug!(Self[B]),
    {
        f(*s)
    }
}

impl<A: Clone> Monad for Vec<A> {
    fn bind<F, B>(f: F, s: Self) -> plug!(Self[B])
    where
        F: Fn(unplug!(Self, A)) -> plug!(Self[B]),
    {
        let res: Vec<B> = s.into_iter().map(|x| f(x)).flatten().collect();
        res
    }
}

impl<A> Monad for Option<A> {
    fn bind<F, B>(f: F, s: Self) -> plug!(Self[B])
    where
        F: Fn(unplug!(Self, A)) -> plug!(Self[B]),
    {
        match s {
            Some(x) => f(x),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    ///wew lad
    fn higher_poly_demo<'a, M: Monad, A: 'a + Clone, B: 'a + Clone, F>(m: M, f: F) -> plug!(M[B])
    where
        M: Plug<A> + Plug<B> + Unplug<A = A>, //+Plug<F>+Plug<Fn(A)->plug!(M[B])>,
        M: Plug<Box<Fn(A) -> plug!(M[B])>>,
        M: Plug<F>,
        F: 'static,
        unplug!(M, F): Plug<A> + Plug<B>,
        plug!(M[B]): Monad + Unplug<A = B> + 'a,
        unplug!(plug!(M[B]), F): Plug<B>,
        F: Fn(A) -> B + 'a,
        //F:Fn(A) -> plug!(M[B]) + Clone,
    {
        let cl = Box::new(move |x| Applicative::pure(f(x)));
        Monad::bind::<Box<Fn(A) -> _>, B>(cl as Box<Fn(A) -> _>, m)
    }

    #[test]
    fn use_higher_poly() {
        let f = |x| x + 1;
        let p1 = Some(5);
        let p2 = vec![5];
        let p3 = Box::new(5);
        assert!(higher_poly_demo(p1, f) == Some(6));
        assert!(higher_poly_demo(p2, f) == vec![6]);
        assert!(higher_poly_demo(p3, f) == Box::new(6));
    }
}
