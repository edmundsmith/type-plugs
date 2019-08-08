use core::*;

pub trait Functor: Unplug + Plug<unplug!(Self, A)> {
    fn map<B, F>(f: F, s: Self) -> plug!(Self[B])
    where
        Self: Plug<B>,
        F: Fn(unplug!(Self, A)) -> B;
}

impl<A> Functor for Box<A> {
    fn map<B, F>(f: F, s: Self) -> plug!(Self[B])
    where
        F: Fn(unplug!(Self, A)) -> B,
    {
        Box::new(f(*s))
    }
}

impl<A> Functor for Vec<A> {
    fn map<B, F>(f: F, s: Self) -> plug!(Self[B])
    where
        F: Fn(unplug!(Self, A)) -> B,
    {
        s.into_iter().map(f).collect()
    }
}

impl<A> Functor for Option<A> {
    fn map<B, F>(f: F, s: Self) -> plug!(Self[B])
    where
        F: Fn(unplug!(Self, A)) -> B,
    {
        s.map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn functor_demo<F: Functor, A, B, C>(
        functor: F,
        fun: impl Fn(A) -> B,
        fun2: impl Fn(B) -> C,
    ) -> plug!(F[C])
    where
        F: Plug<A> + Plug<B> + Plug<C> + Unplug<A = A>,
        unplug!(F, F): Plug<A> + Plug<B> + Plug<C>,
    {
        let cmp = |x| fun2(fun(x));
        Functor::map(cmp, functor)
    }

    #[test]
    fn use_functor() {
        let functor = Some("2");
        let f1 = |x: &str| x.parse::<u8>().unwrap();
        let f2 = |x| x as u32 + 1;
        assert_eq!(functor_demo(functor, f1, f2), Some(3));
    }
}
