use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const WATER_PUMP: u8 = 1;

pub fn start_pump() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(WATER_PUMP)?.into_output();
    pin.set_high();
    
    Ok(())
}

pub fn stop_pump() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(WATER_PUMP)?.into_output();
    pin.set_low();

    Ok(())
}

pub fn hydrate_plant() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(WATER_PUMP)?.into_output();
    pin.set_low();

    for _ in 0..5 {
        pin.set_high();
        thread::sleep(Duration::from_secs(1));
        pin.set_low();
        thread::sleep(Duration::from_secs(2));
    }

    Ok(())
}