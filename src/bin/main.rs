#![no_std]
#![no_main]

// use lib::DisplayDriver;

use esp_hal::clock::CpuClock;
use esp_hal::dma_tx_buffer;
use esp_hal::gpio::OutputConfig;
use esp_hal::lcd_cam::lcd::i8080::{Config, TxEightBits, I8080};
use esp_hal::lcd_cam::{BitOrder, ByteOrder, LcdCam};
use esp_hal::time::Rate;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};
use esp_println::println;
use hello_world::display::DisplayDriver;
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
    let rst = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());

    // Define the Data/Command select pin as a digital output
    // let dc = Output::new(peripherals.GPIO7, Level::High, OutputConfig::default());
    // let wr = Output::new(peripherals.GPIO8, Level::High, OutputConfig::default());
    let _rd = Output::new(peripherals.GPIO9, Level::High, OutputConfig::default());

    // Turn on backlight, lcd
    let _backlight = Output::new(peripherals.GPIO38, Level::High, OutputConfig::default());
    let _lcd_on = Output::new(peripherals.GPIO15, Level::High, OutputConfig::default());

    // Define the pins used for the parallel interface 
    let tx_pins = TxEightBits::new(
        peripherals.GPIO39,
        peripherals.GPIO40,
        peripherals.GPIO41,
        peripherals.GPIO42,
        peripherals.GPIO45,
        peripherals.GPIO46,
        peripherals.GPIO47,
        peripherals.GPIO48,
    );

    // Create a DMA interface for sending commands
    let lcd_cam = LcdCam::new(peripherals.LCD_CAM);
    let lcd_config = Config::default().with_frequency(Rate::from_mhz(20));
    let mut i8080 = I8080::new(
        lcd_cam.lcd,
        peripherals.DMA_CH0,
        tx_pins,
        lcd_config,
    ).unwrap()
    .with_ctrl_pins(peripherals.GPIO7, peripherals.GPIO8)
    .with_cs(peripherals.GPIO6);
    
    i8080.set_bit_order(BitOrder::Inverted);
    i8080.set_byte_order(ByteOrder::Inverted);


    // Create a DMA buffer to hold pixel data
    let tx_buf = dma_tx_buffer!(198800).unwrap();

    let mut interface = DisplayDriver::init(tx_buf, i8080);

    let mut delay = Delay::new();

    let _ = interface.clear();

    delay.delay_millis(4000);

    // Define the display from the display bus and initialize it
    let mut display = Builder::new(ST7789, interface)
        .reset_pin(rst)
        .display_offset(35, 0)
        .display_size(170, 320)
        .invert_colors(ColorInversion::Inverted)
        .init(&mut delay)
        .unwrap();

    loop {
        delay.delay_millis(1000);
        println!("Clearing display");
        display.clear(Rgb565::RED).unwrap();

    }
}