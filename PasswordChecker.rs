/// Aditya Mukerjee
use std::io::stdin;
fn main(){
   let mut password = "Skibidi"; // correct password 
   let mut user =  String::new(); // user input held here
   println!("ENTER PASSWORD:");
   stdin().read_line(&mut user).expect("ERROR: INVALID USER ACTION"); // get user input
   // handle incorrect password, note that the trim is important.
   while(user.trim()!=password){
    user.clear();
    println!("ACCESS DENIED");
    println!("INCORRECT PASSWORD PLEASE TRY AGAIN:");
    stdin().read_line(&mut user).expect("ERROR: INVALID USER ACTION");
   }
   // handle correct passwords
   println!("ACCESS GRANTED");
   println!("TO CHANGE PASSWORD RECOMPILE PROGRAM");
}