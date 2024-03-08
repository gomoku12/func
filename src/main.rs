fn main() {
    println!("Hello, world!");

    another_function(128);

    let y = {
        let x = 5;
        x + 1 //文末に;がない
    };
    println!("The value of y is {}", y);

    let w = plus_one(1023);
    println!("1023 + 1 = {}", w);

    let condition = false;
    let number = if condition { 128 } else { 0 };
    println!("The number is {}", number);
}

fn another_function(z: i32) {
    println!("The value of z is {}", z);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
