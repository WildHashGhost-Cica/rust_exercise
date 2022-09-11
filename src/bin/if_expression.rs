fn main(){
    let hello: i32 = 5;

    if hello > 3 {
        println!("Hello World!");
    } else if hello == 5 {
        println!("It's equal with 5!");
    } else {
        println!("Good bye!");
    }

    let age = 20;
    
    let mut name = "";
    
    if age > 25 {
        name = "Jon";
    } else {
        name = "Mike";
    }
    println!("Hello {}", name);

    let age: &str = if age > 18 {
        "much older"
    } else {
        "much younger"
    };

    println!("User is: {}", age);
    
   
}