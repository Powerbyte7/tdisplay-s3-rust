use esp_hal::{dma::{DmaError, DmaTxBuf}, lcd_cam::lcd::i8080::{Command, I8080}, Blocking};
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

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ST7789Command {
    NOP          = 0x00,
    SWRESET		 = 0x01,
    RDDID		 = 0x04,
    RDDST		 = 0x09,

    RDDPM		 = 0x0A,      // Read display power mode
    RDDMADCTL	 = 0x0B,      // Read display MADCTL
    RDDCOLMOD	 = 0x0C,      // Read display pixel format
    RDDIM		 = 0x0D,      // Read display image mode
    RDDSM		 = 0x0E,      // Read display signal mode
    RDDSR		 = 0x0F,      // Read display self-diagnostic result (ST7789V)

    SLPIN		 = 0x10,
    SLPOUT		 = 0x11,
    PTLON		 = 0x12,
    NORON		 = 0x13,

    INVOFF		 = 0x20,
    INVON		 = 0x21,
    GAMSET		 = 0x26,      // Gamma set
    DISPOFF		 = 0x28,
    DISPON		 = 0x29,
    CASET		 = 0x2A,
    RASET		 = 0x2B,
    RAMWR		 = 0x2C,
    RGBSET		 = 0x2D,      // Color setting for 4096, 64K and 262K colors
    RAMRD		 = 0x2E,

    PTLAR		 = 0x30,
    VSCRDEF		 = 0x33,      // Vertical scrolling definition (ST7789V)
    TEOFF		 = 0x34,      // Tearing effect line off
    TEON	     = 0x35,      // Tearing effect line on
    MADCTL		 = 0x36,      // Memory data access control
    IDMOFF		 = 0x38,      // Idle mode off
    IDMON		 = 0x39,      // Idle mode on
    RAMWRC		 = 0x3C,      // Memory write continue (ST7789V)
    RAMRDC		 = 0x3E,      // Memory read continue (ST7789V)
    COLMOD		 = 0x3A,

    RAMCTRL		 = 0xB0,      // RAM control
    RGBCTRL		 = 0xB1,      // RGB control
    PORCTRL		 = 0xB2,      // Porch control
    FRCTRL1		 = 0xB3,      // Frame rate control
    PARCTRL		 = 0xB5,      // Partial mode control
    GCTRL		 = 0xB7,      // Gate control
    GTADJ		 = 0xB8,      // Gate on timing adjustment
    DGMEN		 = 0xBA,      // Digital gamma enable
    VCOMS		 = 0xBB,      // VCOMS setting
    LCMCTRL		 = 0xC0,      // LCM control
    IDSET		 = 0xC1,      // ID setting
    VDVVRHEN     = 0xC2,      // VDV and VRH command enable
    VRHS		 = 0xC3,      // VRH set
    VDVSET		 = 0xC4,      // VDV setting
    VCMOFSET	 = 0xC5,      // VCOMS offset set
    FRCTR2		 = 0xC6,      // FR Control 2
    CABCCTRL	 = 0xC7,      // CABC control
    REGSEL1		 = 0xC8,      // Register value section 1
    REGSEL2		 = 0xCA,      // Register value section 2
    PWMFRSEL	 = 0xCC,      // PWM frequency selection
    PWCTRL1		 = 0xD0,      // Power control 1
    VAPVANEN	 = 0xD2,      // Enable VAP/VAN signal output
    CMD2EN		 = 0xDF,      // Command 2 enable
    PVGAMCTRL	 = 0xE0,      // Positive voltage gamma control
    NVGAMCTRL	 = 0xE1,      // Negative voltage gamma control
    DGMLUTR		 = 0xE2,      // Digital gamma look-up table for red
    DGMLUTB		 = 0xE3,      // Digital gamma look-up table for blue
    GATECTRL	 = 0xE4,      // Gate control
    SPI2EN		 = 0xE7,      // SPI2 enable
    PWCTRL2		 = 0xE8,      // Power control 2
    EQCTRL		 = 0xE9,      // Equalize time control
    PROMCTRL	 = 0xEC,      // Program control
    PROMEN		 = 0xFA,      // Program mode enable
    NVMSET		 = 0xFC,      // NVM setting
    PROMACT		 = 0xFE,      // Program action
}

impl From<ST7789Command> for Command<ST7789Command> {
    fn from(value: ST7789Command) -> Self {
        Command::One(value)
    }
}

impl From<ST7789Command> for u16 {
    fn from(value: ST7789Command) -> Self {
        value as u16
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

        let transfer = display.send(ST7789Command::RAMWR, 0, buffer).unwrap();
        
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

        let transfer = display.send(ST7789Command::RAMWR, 0, buffer).unwrap();
        
        let error;
        (error, display, buffer) = transfer.wait();
        self.display = Some(display);
        self.buffer = Some(buffer);

        error
    }
}
