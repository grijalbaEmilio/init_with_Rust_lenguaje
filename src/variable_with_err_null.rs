
fn vari_err(){
    let num_in_string = "59m";
    let option_error: Result<i32, _> = num_in_string.parse();
    let option_null: Option<i32> = num_in_string.parse().ok();

    match option_error{
        Ok(value)=>{
            println!("the value is: {value}");
        },
        Err(err) => {
            println!("the error is {:?}", err)
        }
    }
}