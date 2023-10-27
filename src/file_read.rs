use std::fs;

fn main(){
    let file_path = "../resources/test.txt";
    let data = read_entire_file_as_string(file_path.to_string());
    let value = match data{
        Ok(value) => value, 
        Err(is_err) => "Asd".to_string()
    };

    println!("value is {}", value);
    
}


//Entire File content as a string

fn read_entire_file_as_string(path: String) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path);
    content
}
