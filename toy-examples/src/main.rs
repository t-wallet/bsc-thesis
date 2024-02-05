use creusot_contracts::*;

#[ensures(result == 42u32)]
fn the_answer() -> u32 {
    42
}

#[ensures(forall<x:u32> result == None ==> v@.len() == 0)]
#[ensures(forall<x:u32> result == Some(x) ==> forall<i:usize> v[i@] <= x)]
fn max(v: &Vec<u32>) -> Option<u32> {
    let mut max;
    match v.get(0) {
        None => { return None; },
        Some(x) => { max = x; },
    }
    //#[invariant()]
    for x in v {
        if x > max {
            max = x;
        }
    }
    Some(*max)
}


/*#[requires(a >= b)]
#[requires(a > 0u32)]
#[requires(b > 0u32)]
#[ensures(result > 0u32)]
#[ensures(a % result == 0u32)]
//#[ensures(b % result == 0u32)]
fn gcd_euclid(mut a: u32, mut b: u32) -> u32 {
    let mut r: u32;
    #[invariant(a > 0u32)]
    #[invariant(b % )]
    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }
    a
}*/


fn main() {}
