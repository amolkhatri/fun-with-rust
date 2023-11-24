use std::env;

fn get_numbers() -> Result<(i32, i32), &'static str>{  //& 'static str is a part of lifetime We will learn about it later on

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {

        let num1:i32 = match args[1].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid number 1 provided {}", args[1]);
                return Err("Invalid first parameter provided");
            }
        };

        let num2: i32 = match args[2].parse(){
            Ok(value) => value,
            Err(_) => {
                println!("Invalid number 2 provided {}", args[2]);
                return Err("Invalid second parameter provided");
            }
        };
        return Ok((num1, num2));
    }
    return Err("No argument provided");

}


fn main(){
    let (a, b) = match get_numbers(){
        Ok(value) => value,
        Err(err) => {
            println!("Err while reading the numbers: {}", err);
            std::process::exit(1);
        }
    };
    // let a = 1;
    // let b = 2;
    let added = add(a, b);
    let substracted = sub(a, b);
    let multiplied = mul(a, b);
    match div(a, b) {
        Some(x) => println!("Result of division is : {} ", x),
        None => println!("Can not divide with 0")
    }
    println!("Added value is : {}", added);
    println!("Subtracted value is : {}", substracted);
    println!("Multiplied value is : {}", multiplied);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> Option<f32> {
    if b == 0 {
        None
    }else{
        let result = (a  as f32) / (b as f32);
        Some(result)
    }
    
}


#[cfg(test)]

mod tests{

    mod add_test {

        use super::super::*;

        #[test]
    
        fn test_add(){
            let num1 = 1;
            let num2 = 2;
            let result = add(num1, num2);
            assert_eq!(result, 3);
        }

        #[test]
        fn add_negative_numbers(){
            let num1 = -1;
            let num2 = -1;
            let result = add(num1, num2);
            assert_eq!(result, -2);

        }
    }

    mod subtract_test{

        use super::super::*;

        #[test]
        fn test_substract(){
            let num1 = 3;
            let num2: i32 = 1;
            let result = sub(num1, num2);
            assert_eq!(result, 2);
        }

    }

    mod multiply_test{

        use super::super::*;

        #[test]
        fn test_multiply(){
            let num1 = 5;
            let num2 = 10;
            let result = mul(num1, num2);
            assert_eq!(result, 50);
        }

        #[test]
        fn test_multiply_with_zero(){
            let num1 = 0;
            let num2 = 10;
            let result = mul(num1, num2);
            assert_eq!(result, 0);
        }
    }


    mod divide_test{
        use super::super::*;

        #[test]
        fn test_divide(){
            let num1 = 10;
            let num2 = 5;
            let result = div(num1, num2);
            assert_eq!(result.unwrap(), 2.0);
        }

        #[test]
        fn test_divide_by_zero(){
            let num1 = 10;
            let num2 = 0;
            let result = div(num1, num2);
            assert_eq!(result, None);
        }

    }

}