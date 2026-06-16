pub fn tutorial() {
    repeat();
}

fn repeat() {
    loop {
        println!("again!")
    }
}

fn first_example() {
    let number = 3;

    if number < 5 {
        println!("\ncondition was true");
    } else {
        println!("\ncondition was false");
    }
}

fn bad_match() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_is_an_expression() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// fn mismatch() {
//     let condition = true;
//
//     let number = if condition { 5 } else { "six" };
//
//     println!("The value of number is: {number}");
// }

fn interesting() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn does_this_even_compile() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
