use std::io;
use colour::{green,red,magenta,blue,cyan};
/*
Simple command line Todo app
*/
fn main() {

let mut my_vec = vec![String::from("Do Exercise"),String::from("Read a book"),String::from("Learn Rust")]; 
    loop
    {
        println!("Enter what you want to do\nList\nNew\nDelete\nQuit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let input=input.to_lowercase().trim().to_string();
        
        if input.eq(&String::from("quit")) 
        {
            cyan!("Bye!, See you next time\n");
            break;
        } 
        else if input.eq(&String::from("list")){
            let mut i=0;
            magenta!("The elements of the vector are\n");
            while i<my_vec.len() {
                blue!("{i}-{}\n",my_vec[i]);
                i=i+1;
            }
        }
        else if input.eq(&String::from("new"))
        {
        green!("Enter the new todo item\n");
        let mut char = String::new();
        io::stdin().read_line(&mut char).expect("Failed to read");
        let char=char.trim().to_string();
        
        my_vec.push(char);
        } 
        else if input.eq(&String::from("delete"))
        {
        red!("Enter the index you want to delete\n");
        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("Failed to read");
        let index=index.trim().parse::<usize>().expect("Failed to read"); 
        red!("The todo '{}' will be removed\n",my_vec[index]);
        my_vec.remove(index);
        }
    }   
    
}