use std::collections::HashMap;

pub fn main() {
    /*
    // */
    array_demo();
    string_demo();
    vector_demo();
    map_demo();
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

fn vector_demo() {
    let mut two_numbers: Vec<i32> = Vec::new();
    two_numbers.push(0);
    two_numbers.push(1);
    let armstrong_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 153, 370, 371, 407];

    println!("binary digits: {:?}", two_numbers);
    println!("There are {} Arsmstrong numbers under 1000:", armstrong_numbers.len());
    for (i,a) in armstrong_numbers.iter().enumerate() {
        println!("{}:\t{}", i+1, a);
    }
}

fn map_demo() {
    // to have hash map with custom key type, Eq and Hash traits must be implemented
    let mut gpa : HashMap<&str, f32> = HashMap::new();
    let default_grade : f32 = -1.0;

    gpa.insert("Ali", 3.5);
    gpa.insert("Vali", 3.7);
    gpa.insert("Vali", 3.0);
    gpa.insert("Gholam", 3.1);
    gpa.insert("Gholi", 3.2);
    gpa.insert("Ghamar", 3.3);

    println!("Map size is: {}", gpa.len());
    println!("We {}have Mali's grade.", if gpa.contains_key("Mali") { "" } else { "don't "} );
    gpa.remove("Ali");
    println!("We don't have Ali's grade anymore! map contains \"Ali\"? --> {}", gpa.contains_key("Ali"));
    println!("gpa of Ali: {}", match gpa.get("Ali") { Some(grade) => grade, None => &default_grade });
    println!("gpa of Vali: {}", match gpa.get("Vali") { Some(grade) => grade, None => &default_grade });
    println!("here are is the grade list:");
    for _ in 0..2 {
        for (k, v) in gpa.iter() {
            println!("{}:\t{}", k, v);
        }
    }
}
