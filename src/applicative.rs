use core::*;
use functor::*;

pub trait Applicative: Functor {
    fn pure(s:Self::A) -> Self;
    fn app<B, F>(f:plug!(Self[F]), s:Self) -> plug!(Self[B]) where
        F:Fn(Self::A) -> B,
        Self:Plug<F>+Plug<B>+Unplug,
        plug!(Self[F]):Unplug<F=Self::F,A=F>+Plug<F>+Clone,
        Self::F:Plug<F>
        ;
}

impl<A> Applicative for Box<A> {
    fn pure(a:A) -> Self {
        Box::new(a)
    }

    fn app<B, F>(f:plug!(Self[F]), s:Self) -> plug!(Self[B])
    where
        F:Fn(Self::A) -> B
    {
        Box::new((*f)(*s))
    }
}

impl<A:Clone> Applicative for Vec<A> {
    fn pure(a:A) -> Self {
        vec![a]
    }
    fn app<B, F>(fs:plug!(Self[F]), s:Self) -> plug!(Self[B])
    where
        F:Fn(Self::A) -> B,
        plug!(Self[F]): Clone,
    {
        let flat:Vec<B> = 
        Functor::map(|x:A|
            Functor::map(|f:F|
                f(x.clone()),
            fs.clone()),
        s).into_iter().flatten().collect();
        flat
    }
}

impl<A> Applicative for Option<A> {
    fn pure(a:A) -> Self {
        Some(a)
    }
    fn app<B, F>(fs:plug!(Self[F]), s:Self) -> plug!(Self[B])
    where
        F:Fn(Self::A) -> B
    {
        match fs {
            Some(f) => match s {
                Some(x) => Some(f(x)),
                None => None
            },
            None => None
        }
    }
}