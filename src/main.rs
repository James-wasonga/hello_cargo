

//  fn main() {
//     let mut s: String = String::from(s:"Hello");

//     change(&mut s);
//     println!("->{s}")
//  }

//  fn change(some_string: &mut String){
//     some_string.push_str(string:",world");
//  }

// fn main(){
//    let _s2 :String = String::from("hello"); 
//  let _s1 = give_ownership();
//  let _s3 = take_and_gives_back(_s2);


// }

// fn give_ownership() -> String{
//    let some_string = String::from("your");
//    some_string
// }

// fn take_and_gives_back(a_string: String) -> String {

//    a_string
// }


// fn main(){
//    let s1 = String::from("hello");
//    let (s2, len) = calculate_length(s1.clone());
//    println!("length of {} is {}",s1,len);

// }

// fn calculate_length(s: String) -> (String,usize){
//    let length = s.len();
//    (s, length)

// }


//reference 
// fn main(){
//    let s1 = String::from("Hello");
//    let len = calculate_length(&s1);
//    println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()


// }


// fn main(){
// let network_response:i32 = 200;
// let is_success: bool = if network_response == 200{true} else{false};

// if is_success {
//    println!("Operation successfull")
// }

//    let mut counter :i32 = 0;
//    loop{
//       counter = counter + 1;
//       println!("I am at num {counter}");

//       if counter == 1 {
//          println!("Starting point")
//       }
//       if counter == 10 {
//          break
//       }
//    }
// }


fn main() {
//  let mut element = [1,2,3,4,5,6,7,8,9];
// //  let mut index = 5;

// //  while index != 0{
// //    println!("looping through index {index} item in array{}",array[index]);
// //    index -=1;
// //  }
// for element in element{
//    println!("The value is: {element}");
// }

for item in 1..20{
   println!("count item {item}");
}
}
