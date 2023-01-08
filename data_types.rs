fn main(){
    // vars
    // integers
    let x = 10_432;         // immuteable  
    // _ works as the visual separator that does effect the value
    let mut y = 5;          //mutebale
    y += 10;                // cahngeable
    println!("{x},{y}");        //println! != println 
 
    // change the data type || type conversion 
    // strongly typed so u32
    // parse auto parses the values that is typed between : and =
    // .except() works like catch becuase
    let x : u32 = "534".parse().expect("please enter the number");
    println!("{x}");

    // u32 u defines unsigned that mean it will it will only store positive number and 0
    // i32 i defines signed that mean it will it will only store both positive and negative number and 0
    // 8, 16, 32, 64, 128, arch are bits that defines the length of the number and the arch asigns the number as the architechture of the system using

    // floating points
    // f of f32 defines floating points

    let decimal : f32 = 43.21;
    println!("{decimal}");


    // boolean

    let flag : bool = true;
    let flag2 = false;
    println!("{flag},{flag2}");

    // tuple
    let tup = (5,4.3,67,-2);
    let tup1 : (u8,f32,u8,i8) = (5,4.3,67,-2);

    // access the variables 
    // tuple unpacking does work
    let (x,y,z,a) = tup;
    println!("{z}");
    println!("{},{}",tup.0,x);

    // println!("{tup}");

    // array



    // 
    let a = "this is some string";
    print!("{a}");                  // println!(a) does not work
    println!("---{}",a.len());      // .len() returns the length of the seq

}