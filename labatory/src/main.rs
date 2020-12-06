fn main() {
    println!("Hello, world!");
}

fn foo(n: i64) -> i64 {
    n.to_string().chars().map(|chr| (chr as i64) - 48).sum()
}
