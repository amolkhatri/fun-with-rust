// Certainly, let's move on to a new problem that will help you understand error handling and working with `Result` and `Option` more deeply in Rust.

// **Problem: File Content Analyzer**

// Write a Rust program that analyzes the contents of a text file. Your task is to open a file, read its contents, and perform some analysis.

// **Specifications**:

// 1. The program should accept the path to a text file as a command-line argument.
// 2. Read the file contents.
// 3. Perform the following analysis:
//    - Count the number of lines in the file.
//    - Count the number of words in the file.
//    - Identify the longest line in the file.
// 4. If the file is found and readable, print the analysis results.
// 5. If the file is not found or not readable, print an appropriate error message.

// **Learning Objectives**:

// - Deepen your understanding of `Result` and error handling.
// - Learn file I/O operations in Rust.
// - Practice working with strings and slices.
// - Use command-line arguments.

// **Hints**:

// - Use `std::env::args()` to handle command-line arguments.
// - Use `std::fs::read_to_string()` to read the file contents.
// - Remember to handle potential errors, like the file not existing or lacking read permissions.
// - Use the `split_whitespace` method for splitting the file's contents into words.
// - Use `lines` method to iterate over lines in the file's contents.

// Try to implement it, and if you encounter any issues or need further guidance, feel free to ask for more specific hints!


use std::env;

fn get_filename(args: Vec<String> ) -> Result<String, &'static str>{
    
    if args.len() > 1 {
        let file_name = &args[1];
        return Ok(file_name.clone());
    }

    return Err("Invalid args provided");

}



fn main(){

}


#[cfg(test)]


mod tests{

    mod get_filename_test {

        use super::super::*;
        #[test]
        fn test_no_args_provided(){
            let args = vec![];
            let result = get_filename(args);
            assert!(result.is_err());
        }

        #[test]
        fn test_returns_file_name(){
            let args: Vec<String>= vec!["exe".to_string(), "file_foo".to_string()];
            let result = get_filename(args);
            assert_eq!(result.unwrap(), "file_foo");
        }
    }
}