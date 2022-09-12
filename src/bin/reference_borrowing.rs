fn main(){
    let mut s = String::from("string");
    {
        let z = &mut s;
        hello(z);
    }
    let t = &mut s;
    
    println!("{}", s)
}

fn hello(a: &mut String) {
    println!("Inside {}", a);
    a.push_str(" suffix");
    
}