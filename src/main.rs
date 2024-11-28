fn main() {
//Understanding Variable Declaration 
//By defult Every thing is Immutable
    let a:i8 = 7;
    let b: i16 = 23;
    let my_str:String =String::from("Hello, World!");
    
    print!("a:{}  b:{}\n",b,a);
    let mut c:bool = false;
//Undersataning the If else Statements 
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

   let n:i8 =22;
   if n ==0{
    print!("{}\n",12);
   }
   else if n==12{
       println!("n Was always 12");
   }
   else {
       println!("I donno what is n")
   }
//Syntax of  Loop
   for _ in -123..-110{
    println!("Loop Dekha");
}
   // This is how i learned to Use Str
   // Here the Variable "my_nud_str" allocated in heep and yes Probably , I am not sure but my_nud_str Contains the adress or the Ownership pointer and yes it's accessable in all by main()

   let my_str2:String = String::from("This Is all i have been waitintg for.");
   let jhut:String = String::from("This seems UnMemoriable");
   println!("Seems Bad {}",jhut);
   print!("{}\n{}\n",my_str2,my_str);

   // Understanding Borrowing

   let mut my_nud_str:String = String::from("Dil Jhoom");

   my_nud_str.push_str("Tumhe Hoor hoor");
   println!("Take   {}  ",my_nud_str);


   
   //Now I want to Create another function to undersstand 2 types of passing by ref

   //Let's first transfer the ownership
    let new_owner:String = my_nud_str;
    // after this onwords there is no defined value for my_nud_str
    println!("Take   {}  ",new_owner);

    //A function like transfer can also transfer the ownership

    let mut got:String=transfer(new_owner);//From this onwords the same new owner is borrowed and will be destroyed after transfer's Exucation

    //The Thing is where i wanna pass by ref the "&" keyword can also be used 

    muting(& mut got);//The got Still exists after this but chnaged. This can be done ,, But i am not sure Why not to multiple  fn

    println!("{}",got);


}   
//This is the way to declare a function with return value
fn transfer(s:String) ->String{
    println!("I transfer is the new Owner of {}",s);
    return s;
    
}
fn muting(mut ad:&mut String) {
    ad.push_str("   Piyu Bole Piya Bole");
    
}