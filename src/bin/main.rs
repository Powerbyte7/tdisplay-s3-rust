#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};
use esp_println::println;
use mipidsi::interface::{Generic8BitBus, ParallelInterface};
use mipidsi::models::ST7789;
use mipidsi::options::ColorInversion;
use mipidsi::Builder;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::RgbColor,
};


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Print over USB Serial
    println!("Hello World!");

    // Define the reset and write enable pins as digital outputs and make them high
    let rst = Output::new(peripherals.GPIO5, Level::Low);
    let _cs = Output::new(peripherals.GPIO6, Level::Low);

    // Define the Data/Command select pin as a digital output
    let dc = Output::new(peripherals.GPIO7, Level::High);
    let wr = Output::new(peripherals.GPIO8, Level::High);
    let _rd = Output::new(peripherals.GPIO9, Level::High);

    // Turn on backlight
    let _backlight = Output::new(peripherals.GPIO38, Level::High);
    let _lcd_on = Output::new(peripherals.GPIO15, Level::High);

    
    // Define the pins used for the parallel interface as digital outputs
    let lcd_d0 = Output::new(peripherals.GPIO39, Level::Low);
    let lcd_d1 = Output::new(peripherals.GPIO40, Level::Low);
    let lcd_d2 = Output::new(peripherals.GPIO41, Level::Low);
    let lcd_d3 = Output::new(peripherals.GPIO42, Level::Low);
    let lcd_d4 = Output::new(peripherals.GPIO45, Level::Low);
    let lcd_d5 = Output::new(peripherals.GPIO46, Level::Low);
    let lcd_d6 = Output::new(peripherals.GPIO47, Level::Low);
    let lcd_d7 = Output::new(peripherals.GPIO48, Level::Low);

    // Define the parallel bus with the previously defined parallel port pins
    let bus = Generic8BitBus::new((
        lcd_d0, lcd_d1, lcd_d2, lcd_d3, lcd_d4, lcd_d5, lcd_d6, lcd_d7,
    ));

    // Define the display interface from a generic 8 bit bus, a Data/Command select pin and a write enable pin
    let di = ParallelInterface::new(bus, dc, wr);

    let mut delay = Delay::new();

    // Define the display from the display bus and initialize 
    // it with the delay struct and the reset pin
    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .display_offset(35, 0)
        .display_size(170, 320)
        .invert_colors(ColorInversion::Inverted)
        .init(&mut delay)
        .unwrap();

    loop {
        delay.delay_millis(100);
        display.clear(Rgb565::RED).unwrap();
        delay.delay_millis(100);
        display.clear(Rgb565::GREEN).unwrap();
        delay.delay_millis(100);
        display.clear(Rgb565::BLUE).unwrap();
    }
}