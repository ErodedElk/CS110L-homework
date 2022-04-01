// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    println!("Welcome to CS110L Hangman!");
    let mut left_count:i32=5;
    let mut mask_char:Vec<char>=Vec::new();
    let mut guessed_words:Vec<char>=Vec::new();

    for i in 0..secret_word.len(){
        mask_char.push('-');
    }

    while left_count>0{
        print_mask_char(&mask_char);
        print_have_guess(&guessed_words);

        print!("You have {} guesses left\nPlease guess a letter:",left_count);
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line.");
        let guess_char : Vec<char> = guess.trim().chars().collect();

        guessed_words.push(guess_char[0]);
        if !(secret_word.contains(guess_char[0])){
            left_count-=1;
            println!("Sorry, that letter is not in the word");
        }
        else {
            for i in 0..secret_word.len(){
                if secret_word_chars[i]==guess_char[0]{
                    mask_char[i]=guess_char[0];
                    if !(mask_char.contains(&'-')){
                        println!("\nCongratulations you guessed the secret word: {}!",secret_word);
                        left_count=-1;
                        break;
                    }
                }
            }
        }
    }
    if  left_count==0{
        println!("\nSorry, you ran out of guesses!");
    }
}

fn print_mask_char(mask:&Vec<char>)
{
    print!("The word so far is ");
    for i in 0..mask.len(){
        print!("{}",mask[i]);
    }
    println!("")
}
fn print_have_guess(guess:&Vec<char>)
{
    print!("You have guessed the following letters:");
    for i in guess.iter(){
        print!("{}",i);
    }
    println!("")
}
