// Const can be global scoope, but you can't move let outside from fn 
const name: &str = "Cica";
const number: i32 = 15;

fn main() {
    let hello = 1;
    // if you want to change hello value you have to add mut or you need declear again your variable 
    let hello = 3 ;

    

    println!("Hello, {}!", name);
    println!("Favorit number is {}", number);
    println!("number three {}", hello);
}
