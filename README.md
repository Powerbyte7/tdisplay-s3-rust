# LilyGo T-Display S3 Rust Template

This is a basic template to get started with development for the LilyGo T-Display-S3 using Rust. 

What's included:
- A custom mipidsi DMA display interface for fast drawing
- esp_println for printing/debugging
- A recent (*though currently unstable*) esp_hal version

## Usage

First set up your development environment as detailed in [this guide](https://esp32.implrust.com/dev-env.html). 

```powershell
cargo install espflash
cargo install espup
espup install
```

After that's done, you should be able to connect your device and call:

```powershell
cargo run
```

