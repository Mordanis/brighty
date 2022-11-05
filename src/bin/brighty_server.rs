use anyhow::Result;
use brighty;

fn main() -> Result<()> {
    let res = brighty::BacklightDeviceServer::new("nvidia_wmi_ec_backlight");
    println!("res is {:?}", res);
    if let Ok(mut server) = res {
        server.start();
    } else if let Err(e) = res {
        eprintln!("Error is {}", e);
        eprintln!("Cause: {}", e.backtrace())
    }
    Ok(())
}
