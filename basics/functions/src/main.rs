fn hello(){         // initialize functions
    println!("Hello!");
}
fn hello_world(){
    println!("Hello World!");
}

fn user(age:u8,name:String){  // functions with parameters
    println!("I 'm {} years old.My name is {}",age,name)
}

fn sum_two_value(number_one:i32,number_two:i32) -> i32{         // function return values
    number_one + number_two
}

fn main(){
    hello();
    hello_world();
    user(27,"Ali".to_string());
    println!("{}",sum_two_value(4,5))
}