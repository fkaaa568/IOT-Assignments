// fn main() {
//     let ab = |x:u32,y:u32|->u32{
//         x+y
//     };
//     println!("The Sum of Two number is :{:?}",ab(20,6));
// }

fn main() {
    let number = |num:u32|-> bool {
    for a in 2..num {
        if num % a == 0 {
            println!("The Number is Non Prime");
            return false; // if it is not the last statement you need to use `return`   
        }
    }
    println!("The Number is Prime");
    true // last value to return
    };
    println!("{:?}",number(2));
}