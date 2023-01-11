fn main() {
    println!("Hello, world!");


    let  x = 5;
    println!("the value of x is  {}", x);
    //x = 6;
    //println!("the value of x is  {}", x);

    
    const MAX_POINTS: u32 = 100_000;
    println!("the value of max point is  {}", MAX_POINTS);




    let x = x+1;
    let x = x*2;


    println!("the value of x is  {}", x);

    let spaces = "     ";
    // is not OK 
    //spaces = spaces.len();
    let spaces = spaces.len();
    println!("the value of spaces is  {}", spaces);

    

    //let f = 2.0;  //f64
    //let f2 : f32 = 3.0; //32


    let sum = 5+10; 
    let difference = 95.5 - 4.3;
    let product = 4* 30;

    let qut = 56.7/32.2;
    let remainder = 43%5;



    println!("the value is  {} {} {} {} {}", sum, difference, product, qut , remainder );


    let t = true;
    let f:bool = false;




    let guess : u32 = "42".parse().expect("Not a number!");
    
    //is not OK
    //let guess = "42".parse().expect("Not a number!");


    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';


    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

}
