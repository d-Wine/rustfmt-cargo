fn main(){
    // immutable variable declaration
    let age = 27;
    println!("My age is {}",age);
    //age = 26; errorline immutable vriable 'age'
    let mut mutage : i32 = 28;
    println!("My age is {}",mutage);
    mutage = 27;
    println!("My actual age after mutable declaration {}",mutage);

    //immutable and constants
    const IS_VIP : bool = true;
    println!("Is VIP user : {}",IS_VIP);

    //let keyword without datatype declare
    let name = "Ali";
    let myage = 27;
    let pi = 3.142;
    println!("{}",name);
    println!("{}",myage);
    println!("{}",pi);

    //const keyword need to declare datatypes
    const NOT_VIP : bool =true;
    //const IS_VVIP = true; errorline constant declare need datatype
    println!("{}",NOT_VIP);

    //shadowing variables
    let x = 26;
    println!("{}",x);
    let x =  x + 1;
    println!("{}",x);
    let x =  x + 3;
    println!("{}",x);
    //shadowing and mutability

}