use std::fmt::Display;

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y:T
}

impl<T>Pair<T>{
    fn new(x:T,y:T)->Self{
        Self{
            x,
            y,
        }
    }
}

impl<T:PartialOrd + Display>Pair<T>{
    fn cmp_Display(&self){

        if self.x >= self.y{
          println!("The Largest Member is x :{}",self.x);
        }
    
    else {
           println!("The Largest Member is y :{}",self.y);

    }
  }
}

#[derive(Debug)]
struct Pairs {
    x: f32,
    y:f32,
}

impl Pairs {
    fn new(x:f32,y:f32)-> Self{
        Self{
            x,
            y,
        }

    }
}

impl Pairs{
    fn cmp_Display(&self){
        if self.x >= self.y{
            println!("The Largest Member is x :{}",self.x);
        }
        else {
            println!("The Largest Member is y :{}",self.y);
        }
    }
}


fn main() {
    
    let Pair_01 = Pair::new(30, 40);
    Pair_01.cmp_Display();

    let Pair_02 = Pairs::new(20.5,2.3);
    Pair_02.cmp_Display();
}

 /*
        In above example of two Types one is with Display and Partial Ord Bound with Generic T
        and other is with specific data type of f32.
        for the comparision values we need to bound it with Display and Partial Ord
        because we can have any type of data in Generic.
        While in second pair we used f32 type which is already Display and Partial Ord type.
        this is we can use only f32 for the comparison of values x & y. but for 1st case
        we can easily compare any type of Numeric of stirng literal values.
    */