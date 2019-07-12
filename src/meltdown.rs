
#[allow(non_camel_case_types, bare_trait_objects)]
pub struct forall_t;

pub struct Concrete<M:Unplug+Plug<A>,A>
{
    pub unwrap:<M as Plug<A>>::result_t
}

impl<M:Unplug+Plug<A>,A> Clone for Concrete<M,A> where <M as Plug<A>>::result_t:Clone, <M as Plug<A>>::result_t:Unplug<F=M,A=A> {
    fn clone(&self) -> Self {
        Concrete::of(self.unwrap.clone())
    }
}

impl<M:Unplug+Plug<A>,A> Concrete<M,A> {
    fn of<MA:Unplug<F=M,A=A>+Plug<A>>(x:MA) -> Self where M:Plug<A, result_t = MA> {
        Concrete { unwrap: x }
    }
}

pub trait Unplug:Sized {
    type F:Unplug+Plug<Self::A>;
    type A;
}

pub trait Plug<A>:Sized {
    type result_t:Plug<A>+Unplug;
}

impl<A,B> Plug<B> for Box<A> {
    type result_t = Box<B>;
}

impl<A> Unplug for Box<A> {
    type F = Self;
    type A = A;
}

impl<A,B> Plug<B> for Vec<A> {
    type result_t = Vec<B>;
}

impl<A,B> Plug<B> for Option<A> {
    type result_t = Option<B>;
}

impl<M:Plug<A>+Plug<B>+Unplug,A,B> Plug<B> for Concrete<M,A> {
    type result_t = Concrete<M,B>;
}

impl<M:Plug<A>+Unplug,A> Unplug for Concrete<M,A> {
    type F = M;
    type A = A;
}

impl<A> Unplug for Vec<A> {
    type F = Vec<forall_t>;
    type A = A;
}

impl<A> Unplug for Option<A> {
    type F = Option<forall_t>;
    type A = A;
}

pub trait Functor: Unplug+Plug<<Self as Unplug>::A> {
    fn map<B, F>(f:F, s:Self) -> <Self as Plug<B>>::result_t where
        Self:Plug<B>,
        F:FnMut(<Self as Unplug>::A) -> B
        ;
}

impl<A> Functor for Concrete<Vec<forall_t>,A> {
    fn map<B,F>(f:F, s:Self) -> <Self as Plug<B>>::result_t where
        F:FnMut(<Self as Unplug>::A) -> B
        //Concrete<Vec<forall_t>,A>:Unplug<A=A,F=Self>+Plug<B,result_t=Concrete<Vec<forall_t>,B>>
    { 
        Concrete::of(s.unwrap.into_iter().map(f).collect())
    }
}

impl<A> Functor for Concrete<Option<forall_t>,A> {
    fn map<B,F>(f:F, s:Self) -> <Self as Plug<B>>::result_t where
        F:FnMut(<Self as Unplug>::A) -> B {
        Concrete::of(s.unwrap.map(f))
    }
}

fn functor_test<F:Functor,A,B,C>(functor:F, fun:impl Fn(A)->B, fun2:impl Fn(B)->C) -> <F as Plug<C>>::result_t where
    F:Plug<A>+Plug<B>+Plug<C>+Unplug<A=A>,
    <F as Unplug>::F: Plug<A> + Plug<B> + Plug<C>
{
    let cmp = |x|fun2(fun(x));
    Functor::map(cmp, functor)
}

pub trait Applicative: Functor {
    fn pure(s:<Self as Unplug>::A) -> Self;
    fn app<B, F>(f:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t where
        F:FnMut(<Self as Unplug>::A) -> B+Clone,
        Self:Plug<F>+Plug<B>+Unplug,
        <Self as Plug<F>>::result_t:Unplug<F=<Self as Unplug>::F,A=F>+Plug<F>+Clone,
        <Self as Unplug>::F:Plug<F>
        ;
}

impl<A:Clone> Applicative for Concrete<Vec<forall_t>,A> {
    fn pure(a:A) -> Self {
        Concrete::of(vec![a])
    }
    fn app<B, F>(fs:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:FnMut(<Self as Unplug>::A) -> B + Clone,
        <Self as Plug<F>>::result_t: Clone,
        {
            let flat:Vec<B> = 
            Functor::map(|x|
                Functor::map(|f|
                    f.clone()(x.clone()),
                fs.clone()),
            s).unwrap.into_iter().map(|x|x.unwrap).flatten().collect();
            Concrete::of(flat)
        }
}

impl<A> Applicative for Concrete<Option<forall_t>,A> {
    fn pure(a:A) -> Self {
        Concrete::of(Some(a))
    }
    fn app<B, F>(fs:<Self as Plug<F>>::result_t, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:FnMut(<Self as Unplug>::A) -> B
        {
            Concrete::of(
                match fs.unwrap {
                    Some(mut f) => match s.unwrap {
                        Some(x) => Some(f(x)),
                        None => None
                    },
                    None => None
                }
            )
    }
}

pub trait Monad : Applicative {
    fn bind<F,B>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where 
        Self:Plug<F>+Plug<B>,
        F:FnMut(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone
    ;
}

impl<A:Clone> Monad for Concrete<Vec<forall_t>,A> {
    fn bind<F,B>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where
        //Self:Plug<F>+Plug<B>,
        F:FnMut(<Self as Unplug>::A) -> <Self as Plug<B>>::result_t + Clone
    {
        let res:Vec<B> = 
            s.unwrap
            .into_iter()
            .map(|x|f.clone()(x).unwrap)
            .flatten().collect();
        Concrete::of(res)
    }
}

#[test]
fn main() {
    let fv = Concrete::of(vec![1,2,3i32]);
    //Concrete::of helps the compiler infer the types through constraint manipulation;
    //simply using the naked constructor might fail to resolve the types
    //let fv = Concrete{unwrap:vec![1,2,3i32]};
    
    let fvr = Functor::map((|x|x as i64+1) as fn(i32)->i64,fv);
    let avr = Applicative::app(Concrete::of(vec![(|x:i64|x+1) as fn(i64)->i64,|x:i64|-x]), fvr);
    println!("{:?}", avr.unwrap);
}