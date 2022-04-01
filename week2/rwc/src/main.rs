use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let line=read_file_lines(filename).unwrap();
    let mut count_line:u32=0;
    let mut count_char:usize=0;
    let mut count_word:usize=0;

    for i in line.iter(){
        count_line+=1;
        count_char+=i.len();
        count_word+=count_word_line(i);
    }
    println!("count_line: {}",count_line);
    println!("lcount_char: {}",count_char);
    println!("count_word: {}",count_word);

}
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {

    let file=File::open(filename)?;
    let mut restlt=Vec::new();
    for line in io::BufReader::new(file).lines()
    {
        let line_str=line?;
        restlt.push(line_str);
    }
    Ok(restlt)
}
fn count_word_line(line:&String)-> usize {
    let mut word_num:usize=0;
    let bytes = line.as_bytes();
    let mut flag:u32=0;
    for &i in bytes.iter(){
        if(i==b' '){
            flag=0;
        }
        else if flag==0 {
            flag=1;
            word_num+=1;
        }
    }
    word_num
}