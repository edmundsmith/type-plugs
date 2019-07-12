use core::*;
use functor::*;

pub trait Applicative: Functor {
    fn pure(a:Self::param_t)->Lifted<Self>;
    fn ap<B>(f: LiftedWith<Self, fn(Self::param_t) -> B>, x: Lifted<Self>) -> LiftedWith<Self, B>;
}

impl<A:Clone> Applicative for Vec<A> {
    fn pure(a:A) -> Lifted<Vec<A>> {
        lift(Box::new(vec![a]))
    }

    fn ap<B>(f: Lifted<Vec<fn(A) -> B>>, x: Lifted<Vec<A>>) -> LiftedWith<Self, B> {
        let fv: Box<Vec<fn(A)->B>> = real(f);
        let xv: Box<Vec<_>> = real(x);
        let rv:Vec<B> = (xv as Box<Vec<A>>).into_iter()
                            .map(|a|fv.iter().map(move|f|f(a.clone())))
                            .flatten().collect();
        lift(Box::new(rv))
    }
}

