fn main(){
    // if - else statement
    let age = 34;
    if age >= 30{
        println!("Age > 30")
    }else{
        println!("Age < 30")
    }
    // if - else if statement
    let name = "John";
    if name == "Jame"{
        println!("Jame logged in!");
    }else if name == "John"{
        println!("John logged in!");
    }else{
        println!("Who logged in!?");
    }
    // if - let statement
    let is_valid = true;
    let the_page = if is_valid {"admin"} else {"user"};
    println!("You see the  {} page",the_page);

    //infinite loop
    //loop{
    //   println!("Hello Rust!")
    //  }

    // return values from loops
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 100{
            break counter;
        }
    };
    println!("{}",result);
    // while loop
    let mut counter = 0;
    while counter != 3{
        counter += 1;
        println!("Counter : {}",counter)
    }
    println!("Loop is terminated!");
    // for loop
    let names = ["John", "Jane", "Ben", "Jack", "Burak", "Ali"];
    for name in names.iter(){
        println!("Name:{}",name)
    }
    //range
    let start = 5;
    let end = 10;
    for number in start ..end{
        println!("{}",number);
    }
}