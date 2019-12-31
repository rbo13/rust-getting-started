fn main() {
    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    //
    // println!("The result is: {}", result)

    while_loop(5);
    for_loop();
    for_range();
}

fn while_loop(mut n: i32) {
    while n != 0 {
        println!("{}:", n);

        n -= 1;
    }

    println!("LIFTOFF");
}

fn for_loop() {
    let col = [1, 2, 3, 4, 5];

    for c in col.iter() {
        println!("The values are: {}", c);
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("{}", number)
    }

    println!("LIFTOFF!!!")
}
