/*use std::collections::HashSet;

fn test<'a>(f: impl Fn(u32, u32) -> bool + 'a) -> impl Iterator<Item=u32> + 'a {
    (0..10).map(move |x|
        (0..10).map(move |y|
            x * y
        ).filter(|&z| f(x, z))
    ).flatten()
}

fn main() {
    let set
    let foo: Vec<_> = test().collect();
    println!("{:?}", foo);
}
*/