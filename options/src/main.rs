// Define a custom error type using an enum
#[derive(Debug)]
enum DivisionError {
    DivideByZero,
}

fn main() {
    let name = Some("Some option string".to_string());
    let this_name: Option<String> = None;
    println!("{}", name.unwrap_or_else(|| "test".to_string()));

    println!(
        "this one is None: {}",
        this_name
            .clone()
            .unwrap_or_else(|| computation(this_name.clone()))
    );

    let val = add(1, -5);

    match val {
        Ok(v) => {
            println!("{}", v);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    let res = divide(10, 2);
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(DivisionError::DivideByZero) => println!("Error: Cannot divide by zero"),
    }

    let res = divide(10, 0);
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(DivisionError::DivideByZero) => println!("Error: Cannot divide by zero"),
    }
}

fn computation(value: Option<String>) -> String {
    match value {
        Some(v) => v,
        None => "Value was None".to_string(),
    }
}

fn add(x: i32, y: i32) -> Result<i32, String> {
    if x < 0 || y < 0 {
        return Err("values are less than 0".to_string());
    };

    Ok(x + y)
}

// Define the divide function with a custom error type
fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    Ok(a / b)
}
