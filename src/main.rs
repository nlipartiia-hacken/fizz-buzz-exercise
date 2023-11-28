fn main() {
    println!("Hello, world!");
    let mut counter = 0;
    for i in 0..301 {
        match (i % 3, i % 5) {
            (0, 0) => {
                println!("fizz buzz");
                counter += 1;
            }
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            (_, _) => (),
        }
    }
    println!("fizz buzz occurred {} times", counter);
}
