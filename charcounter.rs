use std::io::stdin;
// Hi, Tsoding, if you see this comment, it means that I'm your fan! I made a dumb code so that I can join your Discord Server. I want be a professional programmer like you, I watch all of your videos and I'm amazed when you made the languages Bang and Porth. I hope someday that I will be a professional programmer like you, and i will make languages like you. I'm on 10th grade. Keep inpiring, Thanks!! (I also use emacs like you!!)
fn main() {
    println!("Enter something!!!");
    let mut chars = String::new();
    let char_in   = stdin().read_line(&mut chars).unwrap();
    print!("No of characters(including spaces): {}", char_in - 2);
}
