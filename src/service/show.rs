use console::style;
use application::state::State;
use application::Temporary;
use Output;
use Input;

pub fn execute() -> Result<(),()> {
    let mut temp: State = State::new(None);
    println!("{}", style("Read source").cyan());
    match temp.read(String::from("read")) {
        Ok(read) => {
            let input: Input = bincode::deserialize(&read).unwrap();
            println!("{:#?}", input)
        }
        Err(_err) => println!("{}", style("-- Not set --").red())
    }

    println!("{}", style("Write sources").cyan());
    match temp.read(String::from("write")) {
        Ok(write) => {
            let output_vec: Vec<Output> = bincode::deserialize(&write).unwrap();
            for output in output_vec {
                println!("{} {:#?}", style("-").cyan(), output);
            }
        },
        Err(_err) => println!("{}", style("-- Not set --").red())
    }
    Ok(())
}
