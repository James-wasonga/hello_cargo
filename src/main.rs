

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


// fn main() {
// //  let mut element = [1,2,3,4,5,6,7,8,9];
// // //  let mut index = 5;

// // //  while index != 0{
// // //    println!("looping through index {index} item in array{}",array[index]);
// // //    index -=1;
// // //  }
// // for element in element{
// //    println!("The value is: {element}");
// // }

// }

// //fn main{
//    for item in 1..20{
//       println!("count item {item}");
//    }
// }


//A program that prints even numbers

// fn main(){
// //   let num  = [1,2,3,4,5,6,7,8,9,10];

// for num in 0 ..10 {

//   if num % 2 == 0{
//    println!("The even num is {}",num);
//   }
// }
// }

// fn main(){
//    let num = [1,2,3,4,5,6,7,8,9,10];
//    let mut b:usize = 0;

//    while b <= 10{
//       if num[b] % 2 == 0 {
//       println!("{}",num[b]);
      
//    } 
//    b+=1;
// }

// }
// fn main(){

// for num in 1 .. 10{
//    if num % 3 == 0{
//    println!("The number is {}",num);
//    }
// }
// }

// fn main(){
//    let a = [5,10,15,20,25];

//    for element in a{
//       if element < 10 {
//          println!("The num is less than 10");
//       }
//       if element > 20 {
//          println!("The num is greater than 20");
//       }
//    }
 
// }

//an array that print out items in reverse from 50 to 1
// fn main(){
// for num in (1 .. 51).rev(){
   
//    println!("{num}");
// }
// }

//using loop
// use std::io;

// fn main() {
//     println!("Enter an integer:");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read the value in the line");

//     let num: i32 = input.trim().parse().expect("Please enter a valid integer");

//     let mut i = 0;
//     loop {
//         if i > num {
//             break;
//         }
//         println!("{}", i);
//         i += 1;
//     }
// }
//  use std::io;

// fn main(){
//   println!("Enter an integer");
  
//   let mut input = String::new();
//   io::stdin().read_line(&mut input).expect("Failed to read the value");

//   let num: i32 = input.trim().parse().expect("Please enter a valid input");
//    for i in 0.. num {
//       println!("{}", i);
//    }

// }

// use std::io;

// fn main() {
//     println!("Enter an integer:");

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     let num: i32 = input.trim().parse().expect("Please enter a valid integer");

//     let mut i = 0;
//     while i <= num {
//         println!("{}", i);
//         i += 1;
//     }
// }

// use std::io;

// fn main() {
    
//     println!("Enter an integer:");

   
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     let num: i32 = input.trim().parse().expect("Please enter a valid integer");

//     let subtraction_result = num - 1;
//     let addition_result = num + 1;
//     let multiplication_result = num * 2; // Multiplying by 2 for demonstration
//     let division_result = num / 2; // Dividing by 2 for demonstration

//     // Print the results
//     println!("Subtraction: {}", subtraction_result);
//     println!("Addition: {}", addition_result);
//     println!("Multiplication: {}", multiplication_result);
//     println!("Division: {}", division_result);
// }

// //program to revert values

// fn main(){
// //  for num in 0 .. 50{
// //    println!("The number is {num}");

// //  }

//  for element in (0 .. 31).rev(){
//    println!("The value is {element}");
//  }

// }


//program to calculate inputs keyed by the users

// use std::io;

// fn main(){

//    println!("Enter First number");
   
 
//  let mut input = String::new();
//  io::stdin().read_line(&mut input).expect("Failed to read the value in the line ");
//  let num1: i32 = input.trim().parse().expect("Enter a valid first number");

//  println!("Enter second number");

//  let mut input = String::new();
//  io::stdin().read_line(&mut input).expect("Failed to read a valid input from the line");
//  let num2: i32 = input.trim().parse().expect("Enter a valid second number");

//  let addition: i32 = num1 + num2;
//  let subtraction: i32 = num1 - num2;
//  let multiplication: i32 = num1 * num2;
//  let division : i32 = num1 / num2;

//  if  num1 < num2 || num2 == 0 {
//    println!("Enter a valid numbers (hint: first number should not be zero or less than )");
//  }else{

//  println!("The addition is {addition}");
//  println!("The subtraction is {subtraction}");
//  println!("The multiplication is {multiplication}");
//  println!("The division is {division}");

//  }
// }

// use std::io;

// // fn main(){

// //    println!("Enter a number")

// //    let mut input = String::new();
// // io::stdin().read_line(&mut input).expect("Failed to read an input");
// // let num: i32 = input.trim().parse().expect("Please enter a valid input");

// // println!("The entered number is {}",num);
// // }

//slicing  
// fn main(){

//    let s = String::from("Hello World");
//   let hello = &s[0..5];
//   let world = &s[6..11];
//   println!("{hello}");

// }


// struct user{
//    name: String,
//    email: String,
//    string_in_count: u32,
// }

// fn main(){

//    let user1 = user {

//       name: String::from("James Wasonga"),
//       email: String::from("jameswasonga22@gmail.com"),
//       string_in_count: 1, 
//    };

//    println!("{}",user1.name);

// }


// An example using a stract 

// fn main(){
//    let result = area(3,4);

//    println!("The area of the rectangle is {}",result);

   
// }

// fn area(length: u32, width: u32) -> u32 {
//    length * width

// }

Rectangle{
   hieght: u32,
   width: u32,
}
fn main (){
  
   let rect1 = Rectangle{
      hieght: 30,
      width: 50,
   };
   let rect2 = Rectangle{
      width: 10,
      hieght: 40,
   };
    
    let rect3 = Rectangle{
      width: 60,
      hieght: 45,

    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}
