use core::*;

pub trait Functor: Unapply+Lift<<Self as Unapply>::unap_t,<Self as Unapply>::param_t> {
    fn fmap<B>(f:fn(Self::param_t) -> B, x: Lifted<Self>) -> LiftedWith<Self, B>;
}

impl<A> Functor for Vec<A> {
    fn fmap<B>(f:fn(Self::param_t) -> B, x: Box<App<Vec<forall_t>,A>>) -> LiftedWith<Self, B> {
        let mb:Vec<B> = (real(x) as Box<Vec<A>>).into_iter().map(f).collect();
        lift(Box::new(mb))
    }
}