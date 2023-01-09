fn main(){
    function();
    let x = function1();
    println!("returned value {}",x);
    println!("another type of return {} ",func());
    let y = func_args(5,6);
    println!("is 5 is less than 6 {}",y);
}

// return type decleation is mendatory if functions returns something

// void function
fn function(){
    println!("called from another fuction");
}

// return type function
fn function1() -> i32{       // -> i32 defines the return type
    println!("Another functions called this one is return able");
    return 5;
}

// there is another way to return 

fn func()-> bool{
    
    // some code goes here

    5%3==0
}

fn func_args(x:u32, y:u32)-> bool {
    x<y
}