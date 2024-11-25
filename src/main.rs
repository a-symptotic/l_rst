fn main() {
    let a:i8 = 7;
    let b: i16 = 23;
    let my_str:String =String::from("Hello, World!");
    
    print!("a:{}  b:{}\n",b,a);
    let mut c:bool = false;
    if c {
        print!("C is a True Bool.");
    }
    else {
        print!("C is not a False(false)Bool\n");
        c = true;
        if c {
        print!("Now C is a true Bool.\n");
        }
   }
   for _ in -123..-110{
    println!("Loop Dekha");}
   
   let my_str2:String = String::from("This Is all i have been waitintg for.");
   let jhut:String = String::from("This seems UnMemoriable");
   println!("Seems Bad {}",jhut);
   print!("{}\n{}\n",my_str2,my_str);

}