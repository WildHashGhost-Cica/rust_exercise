fn main(){
    let hello: u32 =5;

    let number: i32 = 10;

    let name: &str = "Cica";

    let world = hello as i32;
   
    let x: f64 = 1.5;
    let y: f32 = 1.0;

    let result = x + 0.2;
    // you cant add two different type of data 
    let result_1 = x + y as f64;

    let result_2 = x as f32 + y ;

    //boolean
    let entry = true;
    
    //chart type , you should use 1 character and '', or name the type like (let character_1: char = 'r')
    let character = 'r';

    //using debug print {:?}

    let text = (5, 2.0, 't');

    let (x, y, z) = text;

    let num = [1,2,3,4,];
    let pets: [&str; 3] = ["dog", "cat", "horse"];
    

    println!("hello {}", hello);

    println!("hello {}", number);

    println!("Hello {}", name);

    println!("Hello {}", world);

    println!("Number: {}", x);

    println!("X and 0.2 together:{}", result);

    println!("X and Y together:{}", result_1);
    println!("X and Y together:{}", result_2);
    println!("{}",entry);
    println!("our single character is: {}", character);

    println!("all details: {:?}", text);
    println!("the value of y: {:?}", y);

    println!("Numbers: {:?}", num);
    println!("My pets: {:?}", pets);
    //print 2. pet
    println!("My 2. favorite pet: {:?}", pets[1]);

}