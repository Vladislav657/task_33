use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn get_vector(file: File) -> Vec<i32>{
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut values: Vec<i32> = Vec::new();
    let mut current: i32;

    for line in lines{
        current = line.unwrap().parse::<i32>().unwrap();
        values.push(current);
    }

    values
}


fn find_min_max_count(values: Vec<i32>) -> (i32, i32){
    let mut min_count: i32 = 0;
    let mut max_count: i32 = 0;

    for i in 1..values.len()-1{
        if values[i-1] < values[i] && values[i] > values[i+1]{
            max_count += 1;
        }

        if values[i-1] > values[i] && values[i] < values[i+1]{
            min_count += 1;
        }
    }
    (min_count, max_count)
}


fn main() -> std::io::Result<()>{
    let file = File::open("EXSTREMUM.IN")?;
    let counts: (i32, i32) = find_min_max_count(get_vector(file));

    let mut res = File::create("EXSTREMUM.OUT")?;
    res.write_all(counts.0.to_string().as_bytes())?;
    res.write_all("\n".as_bytes())?;
    res.write_all(counts.1.to_string().as_bytes())?;

    println!("min_count = {}, max_count = {}", counts.0, counts.1);

    Ok(())
}
