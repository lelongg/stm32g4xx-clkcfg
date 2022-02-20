use std::io::stdout;

use embedded_time::rate::*;
use stm32g4xx_clkcfg::{ClockRequirements, Frequencies};

fn main() {
    println!("Default configuration:");
    let req = ClockRequirements::new();
    match req.resolve(false) {
        Ok((cfg, freqs)) => {
            cfg.write_code(&mut stdout()).unwrap();
            print_frequencies(freqs);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    println!("\nConfiguration for 170 MHz sysclk from a 24 MHz crystal:");
    let req = ClockRequirements::new()
        .hse(24_000_000.Hz())
        .sysclk(170_000_000.Hz());
    match req.resolve(false) {
        Ok((cfg, freqs)) => {
            cfg.write_code(&mut stdout()).unwrap();
            print_frequencies(freqs);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    println!("\nConfiguration for the highest possible sysclk with a 48MHz USB clock from a 24 MHz crystal:");
    let req = ClockRequirements::new()
        .hse(24_000_000.Hz())
        .sysclk(170_000_000.Hz())
        .usb_rng_clk(true);
    match req.resolve(false) {
        Ok((cfg, freqs)) => {
            cfg.write_code(&mut stdout()).unwrap();
            print_frequencies(freqs);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn print_frequencies(freqs: Frequencies) {
    println!("Frequencies:");
    println!("  SYSCLK: {:.3} MHz", freqs.sysclk / 1000000.0);
    println!("  HCLK: {:.3} MHz", freqs.hclk / 1000000.0);
    println!("  PCLK1: {:.3} MHz", freqs.pclk1 / 1000000.0);
    println!("  PCLK2: {:.3} MHz", freqs.pclk2 / 1000000.0);
    println!("  48MHz for USB: {:.3} MHz", freqs.usb_rng_clk / 1000000.0);
}
