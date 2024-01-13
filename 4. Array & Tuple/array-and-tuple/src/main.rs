fn main() {
    // array
    let students: [&str; 3] = ["Jhon", "Maria", "Alex"];

    println!("Student on array index 1 is : {}", students[1]);

    // tuple
    let maria = ("Maria", 17, true);
    // print all tuple
    println!("{:?}", maria);

    // print first tuple item
    println!("Student Name : {}", maria.0);
}
