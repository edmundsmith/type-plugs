
///Public empty type to use as the marker for an unapplied type argument
///Effectively, for a given T<_>, T<forall_t> can be considered a rust-friendly
///representation of T: * -> *
#[allow(non_camel_case_types)]
pub struct forall_t;

///This module implements the isomorphism between `M<A>` and `App<M', A>`,
///where `M'` is a marker type for the 'unapplied' `M`. Here, we use
/// M' = M<forall_t>;
///This gives a core of
///M<A> --lift--> App<M',A>
///App<M',A> --real--> M<A>
///The M<A> <=> App<M',A> correspondence is closed by
///M<A> --unap--> M'
///to infer and fix the type M' from lift(ma)
///and the law ∀x: (* → *). (REAL ∘ LIFT_{UNAP x}) ≡ id_x

///Self:LiftsTo<M',A> where Self:UnappliesTo<M'> -- assumes Self=M
///Defines the lift functor into App<M', A> per M<A>
///Can be read as 'Self lifts into (M, A)'
pub trait Lift<M,A> where Self:Sized+Unapply {
    fn lift(ma:Box<Self>) -> Box<App<M,A>> {
        unsafe { Box::from_raw(core::mem::transmute::<*mut (), *mut App<M,A>>(Box::into_raw(ma) as *mut ())) }
    }
}

///Self:RealizesTo<M<A>> -- assumes Self=A
///Defines the real functor into M<A> per A
///Can be read as 'Self is the Real part of MA'
pub trait Real<MA:Unapply> where Self:Sized {
    fn real(a:Box<App<<MA as Unapply>::unap_t,Self>>) -> Box<MA> {
        unsafe { Box::from_raw(core::mem::transmute::<*mut (), *mut MA>(Box::into_raw(a) as *mut ())) }
    }
}

///Self:UnappliesTo<M'> -- assumes Self=M
///Defines the unap functor into M' per M<A>
pub trait Unapply: Sized {
    type unap_t:Sized;
    type param_t:Sized+Real<Self>;
}

///lift(m(a)) ↦ app(m,a) where ∃Lift:m a -> (m, a)
pub fn lift<MA,M,A>(ma:Box<MA>) -> Box<App<M,A>> where MA:Lift<M,A> {
    <MA as Lift<M,A>>::lift(ma)
}


pub fn real<MA:Unapply,A>(a:Box<App<<MA as Unapply>::unap_t,A>>) -> Box<MA> where A:Real<MA> {
    <A as Real<MA>>::real(a)
}

pub struct App<MA,A> ( std::marker::PhantomData<MA>, std::marker::PhantomData<A>);

pub type Lifted<T> = Box<App<<T as Unapply>::unap_t, <T as Unapply>::param_t>>;
pub type LiftedWith<T, A> = Box<App<<T as Unapply>::unap_t, A>>;

trait Monad: Unapply + Lift<<Self as Unapply>::unap_t,<Self as Unapply>::param_t> {
    ///bind : ((f:A -> M<B>), ma: M<A>) -> M<B>
    fn bind<B,V>(f:fn(Self::param_t)-> LiftedWith<Self, B>,
                 ma:Lifted<Self>)
        -> Box<App<<Self as Unapply>::unap_t,B>> where
            B:Real<V>,
            V:Lift<<Self as Unapply>::unap_t,B>,
            B:Real<Self>;

    fn pure(a: Self::param_t) -> Lifted<Self>;
}

impl<A> Lift<Vec<forall_t>,A> for Vec<A>{}
impl<A> Real<Vec<A>> for A {}
impl<A> Unapply for Vec<A> {
    type unap_t = Vec<forall_t>;
    type param_t = A;
}

impl<A,B> Lift<fn(A) -> forall_t, B> for fn(A) -> B {}
impl<A,B> Real<fn(A) -> B> for B {}
impl<A,B> Unapply for fn(A)->B {
    type unap_t = fn(A)->forall_t;
    type param_t = B;
}

impl<A> Monad for Vec<A> {
    fn bind<B,V>(f:fn(A)->LiftedWith<Vec<A>,B>, ma:Lifted<Vec<A>>)
            -> LiftedWith<Vec<A>,B> {
        let realma:Box<Vec<A>> = real(ma);
        let mapped:Vec<B> = realma.into_iter().map(|x|(real(f(x)) as Box<Vec<B>>).into_iter()).flatten().collect();
        lift(Box::new(mapped))
    }
    
    fn pure(a:A)->Lifted<Vec<A>> {
        let mut v = Vec::new();
        v.push(a);
        lift(Box::new(v))
    }
}

pub fn main()
{
    let a:Vec<i32> = vec![1,2,3];
    let ma = lift(Box::new(a));
    //let mb = Monad::bind(|i:i32|lift(Box::new(vec![i,-i])), ma);
    //let b = real(mb);
    //println!("{:?}",b);
}
/*
exists<'a> F<'a> <=> for<R> FnOnce(for<'a> FnOnce(F<'a>) -> R) -> R

let e = move |f| { f("Hello world") }
// this would work had Rust supported polymorphic lambdas

Usage would then become:

e(|s| {
    println!("Hello world {}", s);
})

*/

/*
struct Exists<T>(T);

impl<T> Exists<T> {
    fn have<R>(self) -> ExistentialTo<T,R> {
        ExistentialTo(self.0, std::marker::PhantomData)
    }
}

struct ExistentialTo<T,R>(T, std::marker::PhantomData<R>);

impl<T,R,F> FnOnce<(F,)> for ExistentialTo<T,R> 
    where F: for<'a> FnOnce(&'a T) -> R {
    type Output = R;
    extern "rust-call" fn call_once(self, f: (F,)) -> Self::Output {
        f.0.call_once((&self.0,))
    }
}

fn log(s:&String) {
    println!("Log {}", s)
}

fn ltid<'a>(x:&'a str) -> &'a str {
    x
}

type log_t = for<'a> FnOnce(&'a str) -> ();
type lid = for<'a> FnOnce(&'a str) -> &'a str;

fn main() {
    let mut e: Exists<String> = Exists("test".to_string());
    let mut eh = e.have().call_once((log,));
}*/