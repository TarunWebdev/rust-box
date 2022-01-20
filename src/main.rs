// //func for printing Type
trait TypeDebug {
    fn print_type(&self);
}
impl<T> TypeDebug for T {
    fn print_type(&self){
        println!("The type is '{}'", std::any::type_name::<T>());
    }
}

fn main() {
    // println!("Hello, world!");

    // // mutability 
    // let mut x = 4;

    // println!("value of x is {}",x);

    // x=5;

    // println!("new value of x is {}",x);
    
    // let mut x = 5.45;

    // println!("The value is {}", x );
    // x.print_type();
    
    // x = "hello"; cant change type : static types

    // //tuples
    // let a = (5 , "Hello" , true);
    // println!("first value is {} , and second is {}", a.0 , a.1);
    // let (val_1 , val_2 , _) = a;
    // println!("first value is {} , and second is {}", val_1 , val_2);
    
    // //arrays (same types)

    // let a = [1,2,3,4];
    // println!("vlaue of arr first element is {} ", a[0]);

    // println!("vlaues of arr are {:?} ", a);

    // let a : [f32;10] = [0.5;10];
    // println!("vlaues of arr are {:?} ", a);

    // // if else flow control

    // let num = 4;
    // if num==5 {
    //     println!("It's 5");
    // } else if num ==4 {
    //     println!("It's 4");
    // }
    // else {
    //     println!("It's neither 5 nor 4");
    // }

    // //Loops

    // let mut x = 2;
    // loop{
    //     x = x*2;
    //     if x > 2000 {
    //         break;
    //     }
    //     println!("value of x rn is {}",x);
    // }
    // let mut y = 2;
    // while y*2 < 2000{
    //     y= y*2;
    //     println!("value of y rn is {}",y);

    // }
    // println!("outside");
    // for y in 0..=10{
    //     println!("y is {}",y);
    // }
    // let z = [1, 2, 3];
    // for val in z {
    //     println!("value is {}", val);
    // }

    // //Match

    // let x = 5;
    // match x {
    //     1 => println!("its 1"),
    //     2 => println!("its 2"),
    //     _ => println!("its invalid"),
    // }

    // let a = true;
    // let b = false;
    // match (a,b){
    //     (true,true) => println!("true and true"),
    //     (true,false) => println!("true and false"),
    //     (false,true) => println!("false and true"),
    //     _ => println!("invalid"),
    // }
}
