use combinations::Combinations;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {   
    //let f: &str = r"(1\d{3})";
    //println!("{:?}", perm(3, vec!['a', 'b', 'c']));
    try_regex();

}  
//{{{
//fn perm(length: usize, characters: Vec<char>) -> Vec<char> { //Permutes all possible 2, 3, and 4 character combos of "10*."  //{{{
//    match length == 1 {
//        true => vec![characters[0]],
//        false => {
//            for i in characters.iter() {
//                println!("{}", i);
//                perm(length-1, 
//            }
//            characters
//            //perm(length-1, vec![characters[2]])
//        }
//    }
//} //}}}
//


fn to_octal() -> io::Result<()> { // {{{
    let file = File::open("octal_nums.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for ch in line.unwrap().chars() {
            let f = match ch {
                '0' => '0',
                '1' => '1',
                '2' => '+',
                '3' => '*',
                '4' => '.',
                '5' => '?',
                '6' => '\\',
                '7' => '|',
                _ => '?',
            };
            print!("{}", f);
        }
        println!();
    }
    Ok(())
} //}}}

fn try_regex() -> io::Result<()> {
    let file = File::open("eight_regex.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        test_regex(&("^".to_owned() + &line.unwrap() + "$"));
    }
    Ok(())
}
    


fn test_regex(reg_str: &str) -> io::Result<()> {
    let file = File::open("nums.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(reg_str).unwrap();

    for line in reader.lines() {
        print!("{:?}", line);
        println!("{}", re.is_match(&line.unwrap()));
        //
        //let a = match &line.Ok() {
        //    true => {
        //        println!("{}", re.is_match(&line.unwrap()));
        //    },
        //    false => {println!("{}", "Error");},
        //};
    }

    Ok(())
}

fn print_nums(power: u32) -> () {
    for i in 0..(2_usize.pow(power)) {
        println!("{:0>4}", format!("{:b}", i));  
    }
}
// }}}
