pub fn main() {
    print_fib_seq_up_to(10);
    let (a, b) = (-14, 24);
    let (g, l) = gcd_and_lcm(a, b);
    println!("gcd of ({},{}) is {}. Their lcm is {}", a,b,g,l);
    println!("({},{}) coprime? --> {}", a,b, if are_co_prime(a,b) { "yes" } else { "no" });
    option_demo();
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

fn answers_of_two_degree_equation(a : f32, b : f32, c : f32) -> Option<(f32, f32)> {
    let delta = b*b - 4.0*a*c;
    if a == 0.0 {
        return if b != 0.0 { Some((-c/b, -c/b)) } else { None };
    } else if delta < 0.0 {
        return None;
    } else if delta == 0.0 {
        let ans = -b/(2.0*a);
        return Some((ans,ans));
    } else {
        let ans1 = (-b+delta.sqrt())/(2.0*a);
        let ans2 = (-b-delta.sqrt())/(2.0*a);
        return Some((ans1, ans2));
    }
}

fn option_demo() {
    let equations = [
            [2.0, -8.0, 6.0], // 2*(x-1)*(x-3) = 0
            [3.0, -6.0, 3.0], // 3*(x-1)*(x-1) = 0
            [3.0, -6.0, 9.0] // 3*(x-1)*(x-1)+6 = 0
    ];

    for coefficient in equations.iter() {
        let (a,b,c) = (coefficient[0], coefficient[1], coefficient[2]);
        match answers_of_two_degree_equation(a,b,c) {
            None => println!("The equation {}*x^2 + {}*x + {} = 0 does not have any rigid real answer.", a,b,c),
            Some((ans1, ans2)) => println!("The answers of {}*x^2 + {}*x + {} = 0 are x={} and x={}.",
                                           a,b,c, ans1, ans2),
        };
    }
}
