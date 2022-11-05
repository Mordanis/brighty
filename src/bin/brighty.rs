use anyhow::Result;
use brighty;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    let direction = match args.get(1) {
        Some(i) => i,
        None => {
            println!("Unable to parse arguments");
            println!("Options are:");
            println!("\tup");
            println!("\tdown");
            println!("\t`value`");
            panic!("Unable to parse command line arguments");
        }
    };
    let command = {
        if direction == "up" {
            brighty::SocketMessage::SetRelativeBrightnessUp
        } else if direction == "down" {
            brighty::SocketMessage::SetRelativeBrightnessUp
        } else if let Ok(i) = direction.parse() {
            brighty::SocketMessage::SetBrightnessAbsolute(i)
        } else {
            panic!("Unable to parse command line");
        }
    };
    let mut client = brighty::BrightnessClient::new(command)?;
    client.send();
    Ok(())
}
