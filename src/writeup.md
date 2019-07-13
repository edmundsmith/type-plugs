# Method for Emulating Higher-Kinded Types in Rust

## Intro

I've been fiddling about with an idea lately, looking at how higher-kinded types can be represented in such a way that we can reason with them in Rust here and now, without having to wait a couple years for what would be a significant change to the language and compiler.

There have been multiple discussions on introducing higher-ranked polymorphism into Rust, using Haskell-style Higher-Kinded Types (HKTs) or Scala-looking Generalised Associated Types (GATs). The benefit of higher-ranked polymorphism is in the name; it would allow higher-level abstractions and pattern expression than just the rank-1 polymorphism we have today.

As an example, currently we can express this type:
```rust
fn example<A,B>(x:Vec<A>, f:Fn(A)->B) -> Vec<B>;
``` 
but we can't express the more generic
```rust
fn example2<
	forall A.M<A>:Container, //imaginary syntax
	A,
	B
	>(x:M<A>, f:Fn(A)->B) -> M<B>;
```
as while we can express `forall A.Vec<A>` there is no way to currently express the type```forall M.forall A.M<A>``` (note: the nesting a `forall` within another `forall` type is what gives us our rank-2-ness of the type).

Haskell solves this problem with Higher-Kinded Types, where not only can we express the type `List` parameterised with `Int` as `List Int :: *`, we can also express and reason about `List` on its own: `List :: * -> *`. Current Rust doesn't allow this, as all generic types must be fully instantiated (i.e. `Vec<i32>` exists, but `Vec` does not).

The GAT approach suggests solving this by allowing generic types within traits, as in
```rust
trait VecGAT {
    type Applied<A> = Vec<A>;
}
```
so now we can express the unapplied `Vec` by our `VecGAT`, and `(Vec) i32` is expressed by `VecGAT::Applied<i32>`. This is a pretty Rust-y solution, and is easy enough to understand, but has the drawback of only really existing on paper, for the time being at least. There is an excellent article on [implementing Monads in Rust with GATs here](https://varkor.github.io/blog/2019/03/28/idiomatic-monads-in-rust.html) - if only we could wait for Rust to have GATs implemented and syntax bikeshedded and semantics satiated.

But alas, this is still a proposal a ways off, and we have navel-gazing to do today.

## First attempts

Rust already has a pretty solid constraint system in its traits and associated types if you know how to (ab)use them. As a teen I spent my evenings ~~contorting the Scala type system~~ revising, so I had a pop at expressing a representation for unapplied generic types:
```rust
struct forall_t; //The 'unapplied' type
//Expressing M<A> as distinct M<forall_t> and A types
struct Lift<MA,A>(PhantomData<MA>,PhantomData<A>);
trait Lower<MA:Unapply> {
    fn (lifted:Lift<
        <MA as Unapply>::unapplied_t,
        <MA as Unapply>::param_t>
    ) -> Self;
}
```
plus an isomorphism between`M<A>` and `Lift<M<forall_t>,A>`.

This proved hard to use, not least because the `Lift` type has zero size. I would employ the wonderfully unsafe `core::mem::transmute` between `Box<M<A>>` and `Box<Lift<M<forall_t>,A>>`, resulting in segfaults once optimisations were turned on. It's also pretty difficult to properly relay the meaning of `Isomorphism` to the compiler when we are working with a new representation, as our assumptions about things like `to . fro = fro . to = id` have to be proven from scratch to make anything useful typecheck.

However, this first failed approach was not entirely fruitless - I ended up writing a trait `Unapply` which would come in handy later.
```rust
trait Unapply {
    type unapplied_t;
    type param_t;
}
impl<A> Unapply for Vec<A> {
     type unapplied_t = Vec<forall_t>;
     type param_t = A;
 }
 ```
For a given type `M:Unapply`, I could *almost* close the isomorphism loop through my lifting and unlifting traits:
`M<A> --[M<A> as Unapply]--> (M<forall_t>, A) --[M<A> as Lower]-->M<A>`. This loop worked when I had a concrete `M<A>` as my starting point to go *to* `(M<forall_t>,A)`, but the inference required inferring `M<A>:Lower`, which I couldn't do *from* `(M<forall_t>,A)`

## Breakthrough #1

So, currently Rust can't express a trait accessible via `MyTrait::GenericTy<A>`. But what about moving the generic parameter left once, to get `MyTrait<A>::GenericTy`?
```rust
trait ReplaceWith<A> {
    type result_t;
}
impl<A,B> ReplaceWith<B> for Vec<A> {
    type result_t = Vec<B>;
}
```
Now we get `<Vec<A> as ReplaceWith<B>>::result_t == Vec<B>`. While the syntax is unwieldy, this works to emulate GATs/HKTs: what might ideally be represented as `Ty::GAT<A>` in a future syntax can today be expressed as `<Ty as GAT_Trait<A>>::result_t`.

## Breakthrough 2

Now we have simple GATs expressible, let's get to working on our representations of higher-kinded types.

Firstly, a given type must be liftable to be supported in our ad-hoc representation system. I'll call this `Unplug`, as we are conceptually operating on some `M<A>` to separate the `M<_>` from the `A`.
```rust
trait Unplug {
    type F; //The representation type of the higher-kinded type
    type A; //The parameter type
}
impl<A> Unplug for Vec<A> {
    type F=Vec<forall_t>; //All unapplied Vecs are represented by
    //Vec<forall_t>
    type A=A;
}
```
And to re-combine a split application (to re-plug an unplugged type):
```rust
trait Plug<A> {
    type result_t;
}
//This is identical to the ReplaceWith trait shown above
impl<A,B> Plug<A> for Vec<B> {
    type result_t = Vec<A>;
}
```
so now we can take a `Vec<A>`, split it into separate `Vec<_>` and `A` types, and re-apply a new type to `Vec<_>` to get a `Vec<B>` if we want to.

This is pretty close to complete for the basics of our representation of higher-kinded types. We can now plug and unplug parameters from HKTs like `Option<_>` and `Vec<_>`, so long as we provide the instances to tell the compiler how.

Next up is our wrapper, so that we can hold values with some type `M<T>` while still being able to reason about the `M<_>` and the `T` separately. Whereas before I tried some pointer/transmute/phantom reference shenanigans, I eventually found that I could just store the underlying value (woops), now that I had a way to express its type given the unplugged types available to the wrapper.
```rust
pub  struct  Concrete<M:Unplug+Plug<A>,A> {
    pub unwrap:<M as Plug<A>>::result_t
}
//Conceptually equivalent to the GAT-syntax
//pub struct Concrete<M,A>{unwrap: M::Plug<A>}
```
(Concrete is a holdover name from when I tried juggling lifted pointers and phantoms, but fits as 'the real value represented by our constructed type `(M<forall_t>,A)`).
This struct has some rather nice properties, once you help the type inference out a bit. To this end, I created a helper function to guide the inference:
```rust
impl<M:Unplug+Plug<A>,A> Concrete<M,A> {
    fn of<MA:Unplug<F=M,A=A>+Plug<A>>(x:MA) -> Self
    where M:Plug<A, result_t = MA> {
        Concrete { unwrap: x }
    }
}
```
The associated types in the signature are used to close the loop of `plug . unplug` and `unplug . plug`, so the compiler recognises that we are working on the same `plug (M<_>,A) == MA` and `unplug MA == (M<_>,A)`. A lot of this feels like working in a weird Prolog-Idris hybrid at times.
The type-inference hinting out of the way, let's take a quick look at use:
```rust
let myVec = vec![1,2,3i32];
//Wrapping
let conc = Concrete::of(myVec);
//Unwrapping
let myVec = conc.unwrap;
```
This should be a zero-cost abstraction! Woohoo!

## Putting it to work

Now we can express higher-kinded types, what do we get?

Well, for our demonstration, I will cook up everyone's favourite burrito\* - the Monad!
(\*this is the last time a monad will be described as a burrito).

In Haskell, the typeclass hierarchy for a Monad goes
`Functor f => Applicative f => Monad f`.
For familiarity's sake, I'll start with `Functor`:
```Haskell
class Functor f where
    fmap :: (a -> b) -> f a -> f b
```
is written in our representation as
```rust
pub trait Functor: Unplug+Plug<<Self as Unplug>::A> {
    //Self is conceptually our haskell-ese "f a"
    fn map<B, F>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where
        Self:Plug<B>,
        F:FnMut(<Self as Unplug>::A) -> B
        ;
}

//Example impl for a represented Vec
impl<A> Functor for Concrete<Vec<forall_t>,A> {
    //remember, Self ~ (Vec<_>, A) ~ "f a"
    fn map<B,F>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:FnMut(<Self as Unplug>::A) -> B 
    {        
        Concrete::of(s.unwrap.into_iter().map(f).collect())
    }
}
```
To show functor-level polymorphism in action, here's a simple compose-then-map function:
```rust
fn  functor_test<F:Functor,A,B,C>(
    functor:F,
    fun:impl  Fn(A)->B,
    fun2:impl  Fn(B)->C
) -> <F as Plug<C>>::result_t
where
    F:Plug<A>+Plug<B>+Plug<C>+Unplug<A=A>,
    <F as Unplug>::F: Plug<A> + Plug<B> + Plug<C>
{
    let cmp =  |x|fun2(fun(x));
    Functor::map(cmp, functor)
}
```
Like magic, not a single type annotation in sight (in the function body). The function signature is a bit unwieldy again, as we need to convince the compiler that our HKT's `Plug` and `Unplug` traits behave in the way that we would expect application and abstraction in true HKTs to behave (closing our loops again).

As for `Applicative`, here's one I made earlier:
```rust
pub trait Applicative: Functor {
    fn pure(s:<Self as Unplug>::A) -> Self;
    fn app<B, F>(
        f:<Self as Plug<F>>::result_t,
        s:Self
    ) -> <Self as Plug<B>>::result_t
    where
        F:FnMut(<Self as Unplug>::A) -> B + Clone,
        Self:Plug<F> + Plug<B> + Unplug,
        <Self as Plug<F>>::result_t:
            Unplug<F=<Self as Unplug>::F,A=F> +
            Plug<F> +
            Clone,
        <Self as Unplug>::F:Plug<F>
    ;
}
```
This one took me a little while to get typechecking. Thankfully, most of the type-astronomy is done in the trait, leaving the impl easier to understand and/or implement.
```rust
impl<A:Clone> Applicative for Concrete<Vec<forall_t>,A> {
    fn pure(a:A) -> Self {
        Concrete::of(vec![a])
    }
    fn app<B, F>(
        fs:<Self as Plug<F>>::result_t,
        s:Self
    ) -> <Self as Plug<B>>::result_t
    where
        F:FnMut(<Self as Unplug>::A) -> B + Clone,
        <Self as Plug<F>>::result_t: Clone,
    {
        let flat:Vec<B> =
            Functor::map(|x|
                Functor::map(|f|
                    f.clone()(x.clone()),
                fs.clone()),
            s)
            .unwrap
            .into_iter()
            .map(|x|x.unwrap)
            .flatten()
            .collect();
        Concrete::of(flat)
    }
}
```
Finally, to round things off, we define a true `Monad` in Rust, complete with type inference and checking (provided you have a suitable signature to hint to the compiler)
```rust
pub trait Monad : Applicative {
    fn bind<F,B>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where
        Self:Plug<F>+Plug<B>,
        F:FnMut(<Self as Unplug>::A) ->
            <Self as Plug<B>>::result_t + Clone
        ;
}  

impl<A:Clone> Monad for Concrete<Vec<forall_t>,A> {
    fn bind<F,B>(f:F, s:Self) -> <Self as Plug<B>>::result_t
    where
        F:FnMut(<Self as Unplug>::A)
            -> <Self as Plug<B>>::result_t + Clone
    {
        let res:Vec<B> =
            s.unwrap
            .into_iter()
            .map(|x|f.clone()(x).unwrap)
            .flatten().collect();
        Concrete::of(res)
    }
}
```
