pub fn main() {
    /*
    // */
    array_demo();
    string_demo();
}

fn array_demo() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
                31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
                73, 79, 83, 89, 97];
    let zeros : [u8; 10] = [0; 10];
    println!("We have {} zeros.", zeros.len());
    println!("There are {} prime number under 100.\n", primes.len());
    for (index, p) in primes.iter().enumerate() {
        println!("{}:\t{}", (index+1), p);
    }
    for _ in 0..2 {
        for p in &primes { // again, if we don't ahve iter or &, there's an error.
            print!("{},",p);
            print!("{},",primes[primes.len()-1]);
        }
    }
    println!("");
}

fn string_demo() {
    let name = String::from("Moez");
    println!("My name is {}", name);
    let mut roster = String::from("Ali,Vali,Mali,Gholam   ,    Gholi,\nGhamar");
    for name in roster.split(",") {
        print!("{},", name.trim());
    }
    println!("");
    println!("Javad {} in the roster.", if roster.contains("Javad") { "is" } else { "is not" });
    roster.push_str(",Javad");
    println!("Javad now {} in the roster.", if roster.contains("Javad") { "is" } else { "is not" });
}
