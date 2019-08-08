#[allow(non_camel_case_types, bare_trait_objects)]
pub struct forall_t;

macro_rules! plug {
    ($t1:ty [ $t2:ty ]) => {
        <$t1 as Plug<$t2>>::result_t
    };
}

macro_rules! unplug {
    ($t:ty, $v:ident) => {
        <$t as Unplug>::$v
    };
}

pub struct Concrete<M: Unplug + Plug<A>, A> {
    pub unwrap: plug!(M[A]),
}

impl<M: Unplug + Plug<A>, A> Clone for Concrete<M, A>
where
    plug!(M[A]): Clone + Unplug<F = M, A = A>,
{
    fn clone(&self) -> Self {
        Concrete::of(self.unwrap.clone())
    }
}

impl<M: Unplug + Plug<A>, A> Concrete<M, A> {
    pub fn of<MA: Unplug<F = M, A = A> + Plug<A>>(x: MA) -> Self
    where
        M: Plug<A, result_t = MA>,
    {
        Concrete { unwrap: x }
    }
}

pub trait Unplug: Sized {
    type F: Unplug + Plug<Self::A>;
    type A;
}

pub trait Plug<A>: Sized {
    type result_t: Plug<A> + Unplug;
}

impl<M: Plug<A> + Plug<B> + Unplug, A, B> Plug<B> for Concrete<M, A> {
    type result_t = Concrete<M, B>;
}

impl<M: Plug<A> + Unplug, A> Unplug for Concrete<M, A> {
    type F = M;
    type A = A;
}
