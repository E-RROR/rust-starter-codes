pub fn run() {

    let mut x: u8 = 1;

    // infinite loop
    loop {
        println!("Hello");
        x = x+1;
        if x > 10 {
            break;
        };
    };

    while x < 100 {
        x += 10;
        println!("Hello While");
    };

    for x in 0..100 {
        println!("{:?}", x);
    }

}
