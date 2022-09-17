fn main() {
 
    // static in stack
    let vari = "world";
    // dinamic in heap
    let x = String::from("this is not &str");
    let mut y = vari;

    let num: i32 = 4;

    let mut num2 = num;

    num2 = 23;

    print!("num1: {num}\nnum2: {num2}\n");

    y = "re assign";

    /* println!("\nvari is : {vari}");
    println!("x is : {x}");
    println!("y is : {y}\n");
    println!() */
    
}

fn use_string(s: &str){
    print!("{s}")
}
