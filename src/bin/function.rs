fn name(first: &str){
    println!("My first name: {}", first)
}

fn add(x : i32) -> i32 {
     return x + 5;
}

fn add_1(x: i32, y: i32) -> i32{
    //if remove the return we can't use ; by the end
    x + y 
}

fn add_2(x: i32, y: i32 ) -> i32{
    let z: i32 = x + y ;
    z
}

fn main(){

    name("Cica");
    
    println!("x and 5: {}", add(5));
    println!("y and 3: {}", add_1(5,6));
    println!("x and y: {}", add_2(5,7));
}