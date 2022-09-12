fn main(){
    let n: i32 = 10;

    p(n);
    println!("{}",n);

    let s: String = hello();
    println!("{}",s);
   
}

fn takes_ownership(s: String){
    println!("inside function {}", s)
}

fn p(n: i32){
    println!("{}",n);
}

fn hello() -> String {
    let a: String = String::from("abc");
    a
}