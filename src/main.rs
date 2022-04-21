#![allow(dead_code)]

pub use prelude::*;

#[macro_use]
mod prelude;

mod game;
mod algo;
mod util;

fn main() {
    let g = CupGame().build();

    let gk = MKBSC::new(g.game.clone()).build();

    println!("G: {:?}", g);

    println!("(G|0)^K: {:?}", gk.origin().gki[0]);
    println!("(G|1)^K: {:?}", gk.origin().gki[1]);

    println!("G^K: {:?}", gk);

    let g2k = MKBSC::new(gk.game.clone()).build();

    println!("(G^K|0)^K: {:?}", g2k.origin().gki[0]);
    println!("(G^K|1)^K: {:?}", g2k.origin().gki[1]);

    println!("G^(2K): {:?}", &g2k);

    let mut strategies = all_strategies(g2k.clone());
    loop {
        /*if strategies.get_raw()
            .map::<_, Vec<_>>(|x|
                x.iter()
                    .map(|a|
                        a.map(|a| a as isize)
                        .unwrap_or(-1)
                    ).collect()
            ) != [vec![0, 2, 2, -1, 1, 1], vec![0, 2, -1, 1, 1]] {
            if !strategies.advance() { break; }
            continue;
        }*/

        let s2k = strategies.get_ref();
        
        println!("\n{:?}", &s2k);

        println!("G^(2K):");
        let result = verify_strategy(&g2k, &s2k);
        println!("{:?}", result);

        if true {
            println!("G^K:");

            let sk = strategies.transducers();
            let result = verify_strategy(&gk, &sk);
            println!("{:?}", result);

            for i in 0..2 {
                let transducer = Transducer::build(&g2k.origin().gi[i], &sk.project(i));
                println!("{}: {:?}", i, transducer);
            }

            println!("G:");

            let s = KBSCStratProfile::new(sk, gk.clone());
            let s: [_; 2] = array_init(|i|
                transducer(
                    gk.origin().gki[i].clone(),
                    s.project(i)
                )
            );
            let result = verify_strategy(&g, &s);
            println!("{:?}", result);

            for i in 0..2 {
                let transducer = Transducer::build(&gk.origin().gi[i], &s.project(i));
                println!("{}: {:?}", i, transducer);
            }
        }

        if !strategies.advance() { break; }
    }
}

/*fn print_type<T>(x: T) {
    println!("{}\n", std::any::type_name::<T>());
}*/
