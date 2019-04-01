pub fn main() {
    print_fib_seq_up_to(10);
    let (a, b) = (-14, 24);
    let (g, l) = gcd_and_lcm(a, b);
    println!("gcd of ({},{}) is {}. Their lcm is {}", a,b,g,l);
    println!("({},{}) coprime? --> {}", a,b, if are_co_prime(a,b) { "yes" } else { "no" });
}

fn print_fib_seq_up_to(n : u8) {
    for i in 0..n {
        println!("fib({}) = {}", i, fibonacci(i as i32));
    }
}

fn fibonacci(n : i32) -> i32 {
    let (mut cur, mut fnp1, mut fnp2) = (n, 1, 0);
    for _ in 1..n {
        cur = fnp1 + fnp2;
        fnp2 = fnp1;
        fnp1 = cur;
    }
    return cur;
}

fn gcd_and_lcm(n : i32, m : i32) -> (i32, i64) {
    let (a,b) : (i32, i32) = (std::cmp::max(m.abs(), n.abs()), std::cmp::min(m.abs(), n.abs()));
    let (mut x, mut y) = (a,b);
    while x%y != 0 {
        let t = x%y;
        x = y;
        y = t;
    }
    return (y, ((a as i64)*(b as i64)/(y as i64)));
}

fn are_co_prime(n: i32, m : i32) -> bool {
    return gcd_and_lcm(n,m).0 == 1;
}
