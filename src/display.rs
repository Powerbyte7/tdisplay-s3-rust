use esp_hal::{dma::{DmaError, DmaTxBuf}, lcd_cam::lcd::i8080::I8080, Blocking};
use mipidsi::interface::Interface;
use core::result::Result;
use core::iter::IntoIterator;

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
}

impl<'d> Interface for DisplayDriver<'d> {
    type Error = DmaError;
    type Word = u8;

    fn send_command(&mut self, command: u8, args: &[u8]) -> Result<(), Self::Error> {
        let mut display = self.display.take().unwrap();
        let mut buffer = self.buffer.take().unwrap();
        
        buffer.fill(args);

        let transfer = display.send(command, 0, buffer).unwrap();

        let error;
        (error, display, buffer) = transfer.wait();
        self.display = Some(display);
        self.buffer = Some(buffer);
        error
    }

    fn send_pixels<const N: usize>(
            &mut self,
            pixels: impl IntoIterator<Item = [Self::Word; N]>,
        ) -> Result<(), Self::Error> {
        let mut display = self.display.take().unwrap();
        let mut buffer = self.buffer.take().unwrap();
        let buffer_mem = buffer.as_mut_slice();

        let mut index = 0;

        for pixel in pixels {
            for word in pixel {
                buffer_mem[index] = word;
                index += 1;
            }
        }

        let transfer = display.send(0x2C as u8, 0, buffer).unwrap();
        
        let error;
        (error, display, buffer) = transfer.wait();
        self.display = Some(display);
        self.buffer = Some(buffer);

        error
    }

    fn send_repeated_pixel<const N: usize>(
            &mut self,
            pixel: [Self::Word; N],
            count: u32,
        ) -> Result<(), Self::Error> {
        let mut display = self.display.take().unwrap();
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

        let transfer = display.send(0x2C as u8, 0, buffer).unwrap();
        
        let error;
        (error, display, buffer) = transfer.wait();
        self.display = Some(display);
        self.buffer = Some(buffer);

        error
    }
}
