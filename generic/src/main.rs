use std::fmt::Display;
fn main() {
   check_password(1878134);
    //as if we can place different type value then its mismatched type value
    //check_password("Karachi");
    let string_pass=String::from("Karachi");
    all_check_password(string_pass);
    all_check_password(123456)


}
fn check_password(x:i32){
    println!("Your Password Is {}", x)
}
//Generic Type Functions
fn all_check_password<T:Display>(x:T){
    println!("Your Password Is {}", x)
}