use anyhow::Result;
use brighty;
use std::io::Read;

fn main() -> Result<()> {
    let brightness_path = get_brightness_dir()?;
    println!("brightness_dir is {:?}", brightness_path);
    let res = brighty::BacklightDeviceServer::new(brightness_path);
    println!("res is {:?}", res);
    if let Ok(mut server) = res {
        server.start();
    } else if let Err(e) = res {
        eprintln!("Error is {}", e);
        eprintln!("Cause: {}", e.backtrace())
    }
    Ok(())
}

fn get_brightness_dir() -> Result<String> {
    let mut brightness_config_file = std::fs::File::options()
        .read(true)
        .open(brighty::CONFIG_FILENAME)?;
    println!("opened brightness path");
    let mut brightness_dir = String::new();
    brightness_config_file.read_to_string(&mut brightness_dir)?;
    println!("brightness config is {:?}", brightness_dir);
    Ok(brightness_dir.replace('\n', ""))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_brightness_can_read() {
        let brightness = super::get_brightness_dir();
        assert!(brightness.is_ok());
    }
}
