extern crate rand;

use rand::distributions::{Alphanumeric, Uniform};
use rand::{thread_rng, Rng};

macro_rules! forc {
    ($id_ret:expr; | $id_in:ident <- $iter:expr) => {{
        let mut v = Vec::new();
        for elem in $iter {
            let $id_in = elem;
            let res = $id_ret;
            v.push(res);
        }
        v
    }};
    ($body:expr; | $id_in:ident <- $iter:expr, when $cond:expr) => {{
        let mut v = Vec::new();
        for elem in $iter {
            let $id_in = elem;
            if $cond {
                let res = $body;
                v.push(res);
            }
        }
        v
    }};
}

struct User {
    name: String,
    age: i32,
}

impl User {
    fn to_tuple(self) -> (String, i32) {
        (self.name, self.age)
    }
}

fn generate_users(n: i32) -> Vec<User> {
    (0..n)
        .map(|_| {
            let mut rng = thread_rng();
            let dist = Uniform::from(1..80);
            let rand_string: String = rng.sample_iter(&Alphanumeric).take(30).collect();
            let rand_n: i32 = rng.sample(&dist);

            User {
                name: rand_string,
                age: rand_n,
            }
        })
        .collect()
}

fn main() {
    let veci = vec![1, 2, 3, 4, 5];
    let res = forc![x + 1; | x <- veci.iter()];
    println!("res = {:?}", res);

    let res = forc![x + 1; | x <- veci.iter(), when x % 2 == 0];
    println!("res = {:?}", res);

    let res = forc![x + 1; | x <- 1..100];
    println!("res = {:?}", res);

    let users = generate_users(100);
    let user_tuples = forc![u.to_tuple(); | u <- users, when u.age > 18];
    for ut in user_tuples.iter() {
        println!("utuple = {:?}", ut);
    }
}
