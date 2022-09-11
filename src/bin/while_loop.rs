fn main(){
    let mut result: i32 = 0;

    let mut counter: i32 = 0;

    /*loop{
        println!("hello");
         
        counter += 1;

        if counter == 10 {
            result = counter * 2;
            break;
        }
    }

    
    println!("{}", result);*/

    while counter < 10 {
        println!("smaller than 10");
        counter += 1;
    }
    for i in 0..=10 {
        println!("hello world {}", i);
        counter += 2;
    }

    let array: [i32; 4] = [1, 2, 3, 4];
    for n in array.iter().rev(){
        println!("hello W {}", n);
    }
}