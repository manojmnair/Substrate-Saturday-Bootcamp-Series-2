//Q4. 🌟 

// Fix the error with the use of define_x
fn main() {
    println!("{}, world", define_x()); 
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}


