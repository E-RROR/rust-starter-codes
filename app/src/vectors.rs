pub fn run() {

    let mut my_array: Vec<i32> = vec![1, 2];

    my_array[0] = 0;

    println!("{:?}", my_array);

    my_array.push(3);
    my_array.push(4);
    my_array.push(5);

    // Slices
    let slices: &[i32] = &my_array[0..1];
    println!("{:?}", slices);

    // Loop through vectors
    for x in my_array.iter() {
        println!("Number : {}", x);
    }

}
