fn main() {
    let a:i8 = 7;
    let b: i16 = 23;
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

    /*for c:i32 in 0..1000{
        a = a + 100;
    };*/
}
