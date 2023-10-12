fn main(){
    let a = 1;
    let b = 2;
    let added = add(a, b);
    let substracted = sub(a, b);
    let multiplied = mul(a, b);
    let divide: f32 = div(a, b) ;
    println!("Added value is : {}", added);
    println!("Subtracted value is : {}", substracted);
    println!("Multiplied value is : {}", multiplied);
    println!("Divided value is : {}", divide);
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

fn div(a: i32, b: i32) -> f32 {
    (a  as f32)/ (b as f32) 
}