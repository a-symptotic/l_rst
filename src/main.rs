use std::io;

//Now i am gonna try to use a structure
struct Tata{
    school:String,
    building:u8,
    month_year:f32
}

struct Details{
    name:String,
    price:u16,
    parient:String,
    quantity:u8
}
fn main() {
//Day 1
//Implementing struct
    let my = Tata{
        school:String::from("Pahanga Nodal High School"),
        building :11,
        month_year : 12.2001,
    };
//Printing The Structure

    println!("The name of my School is {}, It has {} buildings and it was made by TATA in {}.",my.school,my.building,my.month_year);
//Creating And using another Struct
    let perfume= Details{
        name:String::from("God-Father"),
        price:449,
        parient:String::from("Beardo"),
        quantity:50
    };
    let golab_jal= Details{
        name:String::from("Rose Water"),
        price:95,
        parient:String::from("Dabur"),
        quantity:100
    };
    

//Printing the requred things
    print!("I have one {} and a {} Pack, I don't regularly use them but there details are as follows.\nThe {} is priced {} rupies\n",golab_jal.name,perfume.name,perfume.name,perfume.price);
    println!("The parient company of {} is {} and i have {}ml of it.",perfume.name,perfume.parient,perfume.quantity);



//Understanding Variable Declaration 
//By defult Every thing is Immutable
    let a:i8 = 7;
    let b: i16 = 23;
    let my_str:String =String::from("Hello, World!");
    
    print!("a:{}  b:{}\n",a,b);
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

   let n:i8 =12;
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
   for i in 0..10{
    print!("{}",i);
    println!("{}Loop Dekha",i);
}
   // This is how i learned to Use Str
   // Here the Variable "my_nud_str" allocated in heep and yes Probably , I am not sure but my_nud_str Contains the adress or the Ownership pointer and yes it's accessable in all by main()

   let my_str2:String = String::from("This Is all i have been waitintg for.");
   let jhut:String = String::from("This seems UnMemoriable");
   println!("Seems Bad {}",jhut);
   print!("{}\n{}\n",my_str2,my_str);

   // Understanding Borrowing // here I presume the 

   let mut my_nud_str:String = my_str2;//After this  i can't passen the value my_str2...

   my_nud_str.push_str("Tumhe Hoor hoor");
   println!("{}",my_nud_str);
   

   
   //Now I want to Create another function to undersstand 2 types of passing by ref

   //Let's first transfer the ownership
    let new_owner:String = my_nud_str;
    // after this onwords there is no defined value for my_nud_str
    println!("Take   {}  ",new_owner);

    //A function like transfer can also transfer the ownership

    let mut got:String=transfer(new_owner);//From this onwords the same new owner is borrowed and will be destroyed after transfer's Exucation
    //This Will Not Work "println!("{}",new_owner)"

    //The Thing is where i wanna pass by ref the "&" keyword can also be used 

    muting(& mut got);//The got Still exists after this but chnaged. This can be done ,, But i am not sure Why not to multiple  fn

    println!("{}",got);

    //For taking Input 

    let mut input:String = String::new();// Declaration of input of type String and  initialize it's value to type string::new()
    println!("Input Your String Here");

    io::stdin().read_line(&mut input).expect("Failed Bro");//the readline function of io of type 'standard input(stdin)' takes the adress to store the input as mutable ref

    //Printing the string

    println!("The Str You have entered is {}",input);
    let last_owner :String =  transfer(input);


    //let's try to get another
    println!("last_owner is the new Owner of {}",last_owner);

    println!("Can you Please enter another");
    
    //Writing this as practice 
    let mut last_input_string:String = String::new();
    io::stdin().read_line(&mut last_input_string).expect("Helani Value Assign Kari");

    println!("That's it for today\t{}",last_input_string);



}   
//This is the way to declare a function with return value
fn transfer(s:String) ->String{
    return s;
    
}
fn muting(ad:&mut String) {
    ad.push_str("   Piyu Bole Piya Bole");
    
}