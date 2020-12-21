use application::state::State;
use application::Temporary;
use Output;
use Input;

pub fn execute() -> Result<(),()> {
    let mut temp: State = State::new(None);
    println!("\n");
    println!("#################");
    println!("#  Read source  #");
    println!("#################");
    match temp.read(String::from("read")) {
        Ok(read) => {
            let input: Input = bincode::deserialize(&read).unwrap();
            println!("{:?}", input)
        }
        Err(_err) => println!("Not set")
    }
    println!("\n");
    println!("#################");
    println!("\n");
    println!("#################");
    println!("# Write sources #");
    println!("#################");
    match temp.read(String::from("write")) {
        Ok(write) => {
            let output_vec: Vec<Output> = bincode::deserialize(&write).unwrap();
            for output in output_vec {
                println!("- {:?}", output);
            }
        },
        Err(_err) => println!("Not set")
    }
    println!("\n");
    Ok(())
}
