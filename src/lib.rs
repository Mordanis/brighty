#![feature(unix_chown)]
use std::os::unix::fs;

use anyhow::Result;
use std::fs::File;
use std::io::Error;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub const SOCKET_PATH: &str = "/run/brighty.socket";
pub const BASE_BRIGHTNESS_PATH: &str = "/sys/class/backlight";

#[derive(Debug)]
pub enum SocketMessage {
    SetBrightnessAbsolute(usize),
    SetRelativeBrightnessUp,
    SetRelativeBrightnessDown,
}

impl SocketMessage {
    fn from_buff(buff: &[u8]) -> Result<Self, Error> {
        if buff.len() < 5 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Input buffer not long enough",
            ));
        }
        if buff[0] == 0 {
            Ok(Self::SetRelativeBrightnessUp)
        } else if buff[0] == 1 {
            Ok(Self::SetRelativeBrightnessDown)
        } else if buff[0] == 2 {
            let mut msg_bits = [0u8; 4];
            for i in 0..4 {
                msg_bits[i] = buff[i + 1];
            }
            let buff_val = u32::from_ne_bytes(msg_bits);
            Ok(Self::SetBrightnessAbsolute(buff_val as usize))
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Unable to parse string :C",
            ))
        }
    }

    fn to_buff(&self) -> [u8; 5] {
        match self {
            Self::SetBrightnessAbsolute(n) => {
                let msg_bits = (*n as u32).to_ne_bytes();
                let mut buffer = [0u8; 5];
                buffer[0] = 2;
                for i in 0..4 {
                    buffer[i + 1] = msg_bits[i]
                }
                buffer
            }
            Self::SetRelativeBrightnessUp => [0u8; 5],
            Self::SetRelativeBrightnessDown => [1u8, 0, 0, 0, 0],
        }
    }
}

#[derive(Debug)]
pub struct BacklightDeviceServer {
    brightness_file: File,
    socket: UnixStream,
    max_brightness: usize,
    current_brightness: usize,
    relative_delta: usize,
}

impl BacklightDeviceServer {
    pub fn new<T: AsRef<std::ffi::OsStr> + AsRef<std::path::Path>>(dir: T) -> Result<Self> {
        let socket_path = std::path::Path::new(SOCKET_PATH);
        let stream = UnixStream::connect(socket_path)?;

        let path = std::path::Path::new(BASE_BRIGHTNESS_PATH).join(dir);
        let brightness_path = path.join("brightness");
        let max_brightness_path = path.join("max_brightness");
        let mut brightness = File::options()
            .read(true)
            .write(true)
            .open(brightness_path)?;
        let mut max_brightness = File::options().read(true).open(max_brightness_path)?;

        let mut raw_brightness_value = String::new();
        brightness.read_to_string(&mut raw_brightness_value)?;
        raw_brightness_value = raw_brightness_value.replace('\n', "");
        let current_brightness = raw_brightness_value.parse()?;

        let mut max_raw_brightness_value = String::new();
        max_brightness.read_to_string(&mut max_raw_brightness_value)?;
        max_raw_brightness_value = max_raw_brightness_value.replace('\n', "");
        let max_brightness_value = max_raw_brightness_value.parse()?;

        let relative_delta = max_brightness_value / 100;

        Ok(Self {
            brightness_file: brightness,
            socket: stream,
            current_brightness,
            max_brightness: max_brightness_value,
            relative_delta,
        })
    }

    pub fn start(&mut self) {
        self.listen_for_commands();
    }

    fn listen_for_commands(&mut self) {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));
            println!("waiting for command!");
            let mut buff = [0u8; 256];
            let res = self.socket.read(&mut buff);
            println!("got buffer {:?}", &buff[0..5]);
            println!("read result is {:?}", res);
            if !res.is_ok() {
                continue;
            }

            if let Ok(msg) = SocketMessage::from_buff(&buff) {
                self.execute_command(msg);
            }
        }
    }

    fn execute_command(&mut self, command: SocketMessage) {
        println!("got command {:?}", command);
        match command {
            SocketMessage::SetRelativeBrightnessUp => {
                self.current_brightness += self.relative_delta;
            }
            SocketMessage::SetRelativeBrightnessDown => {
                if self.relative_delta < self.current_brightness {
                    self.current_brightness -= self.relative_delta;
                } else {
                    self.current_brightness = 0;
                }
            }
            SocketMessage::SetBrightnessAbsolute(n) => {
                self.current_brightness = n;
            }
        }

        if self.current_brightness > self.max_brightness {
            self.current_brightness = self.max_brightness;
        }
        self.set_brightness();
    }

    fn set_brightness(&mut self) {
        write!(self.brightness_file, "{}", self.current_brightness);
    }
}

#[derive(Debug)]
pub struct BrightnessClient {
    command: SocketMessage,
    socket: UnixStream,
}

impl BrightnessClient {
    pub fn new(command: SocketMessage) -> Result<Self> {
        let socket = UnixStream::connect(SOCKET_PATH)?;
        Ok(Self { command, socket })
    }

    pub fn send(&mut self) -> Result<()> {
        let cmd = self.command.to_buff();
        self.socket.write(&cmd)?;
        Ok(())
    }
}
