use std::fs;
use std::fs::File;
use std::io::prelude::*;


fn main(){
    let file_path = "../resources/test.txt";
    let data = read_entire_file_as_string(file_path.to_string());
    let value = match data{
        Ok(value) => value, 
        Err(_) => "Asd".to_string()
    };

    println!("value is {}", value);

    let data_2 = read_file_using_file_object(file_path.to_string());

    let value_2 = match data_2 {
        Ok(value) => value,
        Err(_) => String::from("")
    };
    
    println!("value_2 is {}", value_2);
}


//Entire File content as a string

fn read_entire_file_as_string(path: String) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path);
    content
}

//Read the file 

fn read_file_using_file_object(path: String) -> Result<String, std::io::Error>{
    let mut file = File::open(path)?;    
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{}", content);
    Ok(content)
}