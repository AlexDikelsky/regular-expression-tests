fn main() {

}

fn test_regex(reg_str: &str) -> io::Result<()> {
    let file = File::open("nums.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(reg_str).unwrap();

    for line in reader.lines() {
        print!("{:?}", line);
        println!("{:?}, {}", re.is_match(&line.unwrap()), reg_str);
    }

    Ok(())
}
