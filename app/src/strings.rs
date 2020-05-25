pub fn run() {

    let mut name = String::from("Hi, im ");

    // Push char to name
    name.push('S');

    // Push String to name
    name.push_str("ina");

    // String length
    println!("Length {}", length=name.len());

    // Check empty status
    println!("Empty Check : {:?}", name.is_empty());

    // Contains substring
    println!("Contains : {:?}", name.contains("Hi"));

    // Replace
    println!("Replaced : {}", name.replace("Hi","Hello"));

    // Test Asserts
    assert_eq!(11, name.len());

    // Loop over string white spaces
    for whtspace in name.split_whitespace(){
        println!("{}", whtspace);
    };

    println!("Last output : {:?}", name);
}
