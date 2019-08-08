use core::*;

#[test]
fn main() {
    let fv = Concrete::of(vec![1, 2, 3i32]);
    //Concrete::of helps the compiler infer the types through constraint manipulation;
    //simply using the naked constructor might fail to resolve the types
    //let fv = Concrete{unwrap:vec![1,2,3i32]};

    let fvr = Functor::map((|x| x as i64 + 1) as fn(i32) -> i64, fv);
    let avr = Applicative::app(
        Concrete::of(vec![(|x: i64| x + 1) as fn(i64) -> i64, |x: i64| -x]),
        fvr,
    );
    println!("{:?}", avr.unwrap);
}
