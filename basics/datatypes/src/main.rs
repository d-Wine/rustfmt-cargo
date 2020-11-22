fn main(){
    
    //integers
    let temperature : i32 = -100;//signed 
    println!("Today's Temperature : {}",temperature);
    let price : u32 = 999;//unsigned
    println!("iPhone's Price : {}",price);
    let age = 22; //i32 by default
    println!("My age is {}",age);
    //

    //floating-point
    let amount = 35.50 ;// f64
    let total : f32 = 400.45;
    println!("float32 {}",amount);
    println!("float64 {}",total);

    //numeric-operations

    //addition
    let sum_example = 19+1;
    let get_difference = 2020-1993;
    let multiplication = 10 * 2;
    let division = 30 / 3;
    let get_mod = 10 % 3;
    println!("Sum {}",sum_example);
    println!("Difference {}",get_difference);
    println!("Multiplication {}",multiplication);
    println!("Division {}",division);
    println!("Reminder {}",get_mod);

    //Boolean
    let is_completed = false;
    let is_send : bool = true;
    println!("{}",is_completed);
    println!("{}",is_send);

    //Character
    let symbol = "#";
    println!("{}",symbol);

    //tuples
    let usr_tuples: (i32,f64,bool) = (2019,1500.76,true); // with datatypes
    let user_tuples = (2019,1500.76,true); // without datatypes
    println!("User tuple is {:?}",user_tuples);
    println!("User tuple is {:?}",usr_tuples);

    //destruct 
    let (register_year,balance,is_customer) = user_tuples;
    println!("User's balance: {}",balance);
    println!("User's Register year: {}",register_year);
    println!("User is customer?: {}",is_customer);

    //access tuple indexes
    println!("1.{}",user_tuples.1);
    println!("0.{}",user_tuples.0);
    println!("2.{}",user_tuples.2);

    //arrays
    let users = ["Ali","Bob","Cargo","Enes"];
    println!("Users: {:?}",users);

    //create arrays
    let numbers : [i32;5] = [1,2,3,4,5]; //type,size
    println!("{:?}",numbers);
    let number = [1;5]; //value,size
    println!("{:?}",number);

    //access array element
    println!("0 {}",numbers[0]);
    println!("1 {}",numbers[1]);
    println!("2 {}",numbers[2]);
    println!("3 {}",numbers[3]);
    println!("4 {}",numbers[4]);
}