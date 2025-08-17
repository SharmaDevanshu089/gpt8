fn main() {
    //creating the int
    let integer_to_be_passed:i32= 5;
    increment_in_place(integer_to_be_passed);
}
fn increment_in_place(integer_that_is_passed: i32){
    print!("{}", integer_that_is_passed);
}