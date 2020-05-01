#![allow(unused)]
macro_rules! x_and_y {
    (x: $e:expr) => { println!("x: {}", $e) };
    (y: $e:expr) => { println!("y: {}", $e) };
}

macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! print_ex {
    ($e:expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    }
}

macro_rules! compr {
    ($id1:ident | $id2:ident <- $lst:expr, $($cond:expr),+) => {
        {
            let mut vec = Vec::new();
            let mut vec_bools: Vec<bool> = Vec::new();
            for num in $lst {
                $(
                    vec_bools.push($cond(num));
                 )+
                if vec_bools.iter().all(|&x| x) {
                    vec.push(num);
                }
                vec_bools = Vec::new();
            }
            vec
        }
    }
}

fn main() {
    x_and_y!(x: 10);
    x_and_y!(y: 10 as f32 / 3.1);
    build_fn!(hello);
    hello();

    print_ex!({
        let y = 20;
        let z = 30;
        z + y + 10 * 3 * 100
    });

    let evens = compr![x | x <- (1..10), |x| x % 2 == 0, |x| x % 4 == 2];
    println!("{:?}", evens);
}
