pub fn main() {
    let mut x = -7;
    let abs = if x < 0 { -x } else { x };
    println!("x is {} and its abs is {}", x, abs);

    if abs > 10 {
        println!("x is a huge number");
    } else if abs >= 5 { // == != < >
        println!("x is a normal number");
    } else {
        println!("x is a tiny number");
    }

    let base : i32 = 2;
    // while true {
    loop { // : equal to while true
        x*=-7;
        println!("x is {}", x);
        if x > base.pow(16) {
            break; // we also have continue, just as it was in C
        }
    }
    
    let (l,h) = (0, 5);
    for i in l..(h*2) {
        println!("{}", i);
    }

    println!("\n\n");
    let range = 2..5;
    for i in range {
        println!("{}", i);
    }

    let animals = vec!["Rabbit", "Dog", "Aligator", "Crow"];
    for _ in 0..2 {
        for (i,animal) in animals.iter().enumerate() {  // if you don't have .iter() it would be
                                        // compile error to use it twice (ownership stuff)
            println!("{}: {}", (i+1), animal);
        }
    }

    let someInt = 3;
    match someInt%6 {
        1 => println!("odd number"),
        0 => {
            println!("even number");
        },
        3 | 5 => println!("odd number"),
        _ => println!("no idea")
    }
}
