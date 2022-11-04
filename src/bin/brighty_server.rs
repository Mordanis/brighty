use anyhow::Result;
use brighty;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let res = brighty::BacklightDeviceServer::new("nvidia_wmi_ec_backlight").await;
    println!("res is {:?}", res);
    if let Ok(mut server) = res {
        server.start().await;
    } else if let Err(e) = res {
        eprintln!("Error is {}", e);
        eprintln!("Cause: {}", e.backtrace())
    }
    Ok(())
}
