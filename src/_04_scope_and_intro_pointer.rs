pub fn main() {
    block_play();
    pointer_intro();
}

fn pointer_intro() {
    let x = 1;
    let xp = &x;
    println!("x is indirectly {} which is directly {}", xp, x);

    let mut y = 1;
    let yp = &mut y;
    *yp += 1;
    println!("y is indirectly {}, it cannot be directly accessed anymore", *yp);

    let mut z = 1;
    {
        let zp = &mut z;
        *zp += 1;
        println!("z is is indirectly {}, it cannot be directly accessed here.", *zp);
    }
    println!("z is is directly {}", z);
}

fn block_play() {
    // we are shadowing y
    let (mut x, y) = (1, 1);
    println!("before the block x={}, y={}", x, y);
    {
        x = 2;
        let y = y+1;
        println!("inside the block x={}, y={}", x, y);
    }
    println!("after the block x={}, y={}", x, y);

    let x = "\"a string now\"";
    println!("after redefining x={}, y={}", x, y);
}
