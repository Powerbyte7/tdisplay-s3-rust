use esp_hal::{dma::{DmaError, DmaTxBuf}, lcd_cam::lcd::i8080::I8080, Blocking};
use mipidsi::interface::Interface;
use core::result::Result;
use core::iter::IntoIterator;
use esp_println::println;

pub struct DisplayDriver<'d> {
    display: Option<I8080<'d, Blocking>>,
    buffer: Option<DmaTxBuf>
}

impl<'d> DisplayDriver<'d> {
    pub fn init(buffer: DmaTxBuf, i8080: I8080<'d, Blocking>) -> DisplayDriver<'d> {
        DisplayDriver {
            display: Some(i8080),
            buffer: Some(buffer)
        }
    }

    pub fn clear(&mut self)  -> Result<(), DmaError> {
        println!("Running clear");

        let display = self.display.take().unwrap();
        let mut buffer = self.buffer.take().unwrap();

        let pixels: [u8; 800] = [144; 800];

        buffer.fill(&pixels);

        let transfer = display.send(0x2a as u8, 0, buffer).unwrap(); // RGB565

        let result = transfer.wait();
        self.display = Some(result.1);
        self.buffer = Some(result.2);

        result.0
    }
}

impl<'d> Interface for DisplayDriver<'d> {
    type Error = DmaError;
    type Word = u8;

    fn send_command(&mut self, command: u8, args: &[u8]) -> Result<(), Self::Error> {
        println!("Running command {:#x}", command);

        let display = self.display.take().unwrap();
        let mut buffer = self.buffer.take().unwrap();

        buffer.fill(args);

        let transfer = display.send(command, 0, buffer).unwrap(); // RGB565

        let result = transfer.wait();
        self.display = Some(result.1);
        self.buffer = Some(result.2);

        result.0
    }

    fn send_pixels<const N: usize>(
            &mut self,
            pixels: impl IntoIterator<Item = [Self::Word; N]>,
        ) -> Result<(), Self::Error> {
            println!("B");
        let display = self.display.take().unwrap();
        let mut buffer = self.buffer.take().unwrap();
        let buffer_mem = buffer.as_mut_slice();

        let mut index = 0;

        for pixel in pixels {
            for word in pixel {
                buffer_mem[index] = word;
                index += 1;
            }
        }

        let transfer = display.send(0x0 as u8, 0, buffer).unwrap();

        let result = transfer.wait();
        self.display = Some(result.1);
        self.buffer = Some(result.2);

        result.0
    }

    fn send_repeated_pixel<const N: usize>(
            &mut self,
            pixel: [Self::Word; N],
            count: u32,
        ) -> Result<(), Self::Error> {
        println!("Pushing framebuffer");
        let display = self.display.take().unwrap();
        let mut buffer = self.buffer.take().unwrap();
        
        buffer.set_length(N*(count as usize));
        
        let buffer_mem = buffer.as_mut_slice();

        let mut index = 0;
        
        for _ in 0..count {
            for word in pixel {
                buffer_mem[index] = word;
                index += 1;
            }
        }

        let transfer = display.send(0x0 as u8, 0, buffer).unwrap();

        let result = transfer.wait();
        self.display = Some(result.1);
        self.buffer = Some(result.2);

        result.0
    }
}
