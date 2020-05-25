pub fn run() {

    let mut my_array: [i32; 2] = [1, 2];

    my_array[0] = 0;

    println!("{:?}", my_array);

    // Slices
    let slices: &[i32] = &my_array[0..1];
    println!("{:?}", slices);

}
