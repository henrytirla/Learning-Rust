/*
Unlike languages such as Ruby and JavaScript,
 Rust will not automatically try to convert non-Boolean types to a Boolean. 
You must be explicit and always provide if with a Boolean as its condition.


*/


fn main() {
    if_else();
	loop_();
	multiple_loop();
}



fn loop_() {
    let mut counter = 0;

    let result = loop {
		println!("Henry Tirla");
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn if_else(){
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



fn multiple_loop() {
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