use std::io;
use rand::prelude::*;

fn main(){
  let guess_list=["bannana","apple","mango","orange","grapes"];
  loop{
  let mut rng=thread_rng();
  let index = rng.gen_range(0..guess_list.len());
  let random_fruit = guess_list[index];
  
  println!("Random Fruit:{}",random_fruit);
  println!("Please enter your guess fruit:");

  let mut input = String::new();
  let mut fruit_name=String::new();

  match io::stdin().read_line(&mut input){
    Ok(_)=>{
      fruit_name = input.trim().to_lowercase().to_string();//input.trim() returns &str
      println!("Selected Fruit: {}",fruit_name);

      if !guess_list.contains(&fruit_name.as_str()){
        println!("Invalid fruit! Please enter a valid fruit");
        continue;
      }
    }
    Err(error)=>{
    eprintln!("Error:{}",error);
    }
  }

  match guses_fruit(&fruit_name,&random_fruit){
    true=>{
      println!("Correct Guess! You are a winner!");
      break;
    }false=>{
      println!("Retry");
    }
  }
  }

}

fn guses_fruit(guess:&str,actual:&str)->bool{
  guess==actual
}
