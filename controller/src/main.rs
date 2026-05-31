use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{
    ops::{Add, Sub},
    cmp::Ord,
};
use fixed::types::I12F20;
use fixed_macro::fixed;
use az::Cast;
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::io::BufReader;
use std::fs::File;
pub fn asymmetrical_slew<T: Add<Output=T> + Sub<Output=T> + Ord + Copy>(current: T, target: T, up_limit: T, down_limit: T) -> T {
    if current > target {
        let diff = current - target;
        let new_delta = core::cmp::min(diff, down_limit);
        current - new_delta
    } else {
        let diff = target - current;
        let new_delta = core::cmp::min(diff, up_limit);
        current + new_delta
    }
}

pub fn slew<T: Add<Output=T> + Sub<Output=T> + Ord + Copy>(current: T, target: T, slew_limit: T) {
    asymmetrical_slew(current, target, slew_limit, slew_limit);
}

const NORMAL_TARGET: i8 = 45;
const AGGRO_TARGET: i8 = 90;
const NORMAL_ROT: i8 = 8;
const AGGRO_ROT: i8 = 20;
const TICK_TRIM: I12F20 = fixed!(0.05: I12F20);

use std::sync::Mutex;

fn trim_xlat(fix: I12F20) -> i8 {
    fix.cast()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TrimFile {
    x_trim: f32,
    y_trim: f32,
}

#[derive(Parser)]
pub struct Opts {
    #[arg(short, long)]
    pub trim_file: Option<String>,
}

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();
    let device_state = DeviceState::new();
    let mut stream = TcpStream::connect("192.168.2.1:4200")?;
    let shared_buf = Mutex::new([0_u8; 7]);

    let mut x_target = 0_i8;
    let mut y_target = 0_i8;
    let mut z_rot = 0_i8;

    let mut x_trim = I12F20::from_bits(0);
    let mut y_trim = I12F20::from_bits(0);
    let mut collective_target = I12F20::from_bits(0);
    let mut aggressive = false;

    if let Some(path) = opts.trim_file {
        let r = BufReader::new(File::open(path).unwrap());
        let trim_file: TrimFile = serde_yaml::from_reader(r).unwrap();
        x_trim = I12F20::from_num(trim_file.x_trim);
        y_trim = I12F20::from_num(trim_file.y_trim);
    }

    std::thread::scope(|s| {
        let handle = s.spawn(|| {
            loop {
                let locked_buf = shared_buf.lock().unwrap();
                let copied_buf: [u8; 7] = locked_buf.clone();
                std::mem::drop(locked_buf);
                println!("Transmitting buf: {:?}", copied_buf);
                stream.write(&copied_buf).unwrap();
                let mut read_buf = [0_u8; 12];
                if let Err(e) = stream.read(&mut read_buf) {
                    println!("Failied to read back orientation data: {:?}", e);
                }
                let x_buf: [u8; 4] = read_buf[0..4].try_into().unwrap();
                let x_orientation = i32::from_be_bytes(x_buf);
                let y_buf: [u8; 4] = read_buf[4..8].try_into().unwrap();
                let y_orientation = i32::from_be_bytes(y_buf);
                let z_buf: [u8; 4] = read_buf[8..12].try_into().unwrap();
                let z_orientation = i32::from_be_bytes(z_buf);
                let x_deg = I12F20::from_bits(x_orientation);
                let y_deg = I12F20::from_bits(y_orientation);
                let z_deg = I12F20::from_bits(z_orientation);
                println!("Orientation: [{}, {}, {}]", x_deg, y_deg, z_deg);
                sleep(Duration::from_millis(50));
            }
        });
        loop {
            if handle.is_finished() {
                match handle.join() {
                    Ok(()) => panic!("Comms thread finished"),
                    Err(e) => std::panic::resume_unwind(e),
                }
            }
            let keys = device_state.get_keys();
            if keys.contains(&Keycode::Key1) {
                aggressive = false;
            }
            if keys.contains(&Keycode::Key2) {
                aggressive = true;
            }
            if keys.contains(&Keycode::Space) {
                if collective_target > 38 {
                    collective_target = std::cmp::min(collective_target + fixed!(0.02: I12F20), fixed!(100: I12F20));
                } else {
                    collective_target = std::cmp::min(collective_target + fixed!(0.5: I12F20), fixed!(100: I12F20));
                }
            } else if keys.contains(&Keycode::N) {
                collective_target = std::cmp::min(collective_target + fixed!(0.75: I12F20), fixed!(50: I12F20));
            } else if keys.contains(&Keycode::LShift) {
                let down_inc = fixed!(0.02: I12F20);
                if collective_target < down_inc {
                    collective_target = fixed!(0: I12F20);
                } else {
                    collective_target -= down_inc;
                }
            } else if keys.contains(&Keycode::C) {
                let down_inc = fixed!(0.1: I12F20);
                if collective_target < down_inc {
                    collective_target = fixed!(0: I12F20);
                } else {
                    collective_target -= down_inc;
                }
            }
            if keys.contains(&Keycode::W) {
                println!("W detected");
                y_target = if aggressive {
                    AGGRO_TARGET
                } else {
                    NORMAL_TARGET
                };
            } else if keys.contains(&Keycode::S) {
                // Haha, I never realized '-if' compiles in rust but it does make intuitive sense
                y_target = -if aggressive {
                    AGGRO_TARGET
                } else {
                    NORMAL_TARGET
                };
            } else {
                y_target = 0;
            }
            if keys.contains(&Keycode::I) {
                y_trim += TICK_TRIM;
            } else if keys.contains(&Keycode::K) {
                y_trim -= TICK_TRIM;
            }
            if keys.contains(&Keycode::D) {
                x_target = if aggressive {
                    AGGRO_TARGET
                } else {
                    NORMAL_TARGET
                };
            } else if keys.contains(&Keycode::A) {
                x_target = -if aggressive {
                    AGGRO_TARGET
                } else {
                    NORMAL_TARGET
                };
            } else {
                x_target = 0;
            }
            if keys.contains(&Keycode::L) {
                x_trim += TICK_TRIM;
            } else if keys.contains(&Keycode::J) {
                x_trim -= TICK_TRIM;
            }
           
            if keys.contains(&Keycode::E) {
                z_rot = if aggressive {
                    AGGRO_ROT
                } else {
                    NORMAL_ROT
                };
            } else if keys.contains(&Keycode::Q) {
                z_rot = -if aggressive {
                    AGGRO_ROT
                } else {
                    NORMAL_ROT
                };
            } else {
                z_rot = 0;
            }
            println!("Collective: {}, X Trim: {}, Y Trim: {}", collective_target, x_trim, y_trim);
            let mut locked_buf = shared_buf.lock().unwrap();
            locked_buf[0] = (x_target + trim_xlat(x_trim)) as u8;
            locked_buf[1] = (y_target + trim_xlat(y_trim)) as u8;
            locked_buf[2] = z_rot as u8;
            let col_buf = collective_target.to_bits().to_be_bytes();
            locked_buf[3..].copy_from_slice(&col_buf);
            std::mem::drop(locked_buf);
            std::thread::sleep(Duration::from_millis(10));
        }
    });
    Ok(())
}
