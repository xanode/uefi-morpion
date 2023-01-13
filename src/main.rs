#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use uefi::proto::console::{
    gop::GraphicsOutput,
    text::{Key, ScanCode},
};
use uefi::table::boot::BootServices;

use uefi_graphics::UefiDisplay;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        PrimitiveStyle, Rectangle,
    },
};


fn draw(bt: &BootServices) {
    // Open graphics output protocol
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

    // Get the framebuffer
    let mode  = gop.current_mode_info();
    let mut fb = gop.frame_buffer();

    // Get resolution
    let (width, height) = mode.resolution();

    let display = &mut UefiDisplay::new(
        fb.as_mut_ptr(),
        mode.stride() as u32,
        (width as u32, height as u32),
        &fb
    );

    // Clean up the screen
    display.clear(Rgb888::BLACK).unwrap();

    // Draw the vertical lines
    let x1 = width / 3;
    let x2 = x1 * 2;
    Rectangle::new(Point::new(x1 as i32, 0), Size::new(1, height as u32))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::new(119, 185, 242), 1))
        .draw(display)
        .unwrap();
    Rectangle::new(Point::new(x2 as i32, 0), Size::new(1, height as u32))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::new(119, 185, 242), 1))
        .draw(display)
        .unwrap();
    
    // Draw the horizontal lines
    let y1 = height / 3;
    let y2 = y1 * 2;
    Rectangle::new(Point::new(0, y1 as i32), Size::new(width as u32, 1))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::new(119, 185, 242), 1))
        .draw(display)
        .unwrap();
    Rectangle::new(Point::new(0, y2 as i32), Size::new(width as u32, 1))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::new(119, 185, 242), 1))
        .draw(display)
        .unwrap();
}


#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let mut drawn = false;
    loop {
        let key = system_table.stdin().read_key().unwrap();
        let _ = match key {
            Some(k) => {
                match k {
                    Key::Special(ScanCode::ESCAPE) => break,
                    _ => {},
                };
            }
            _ => if !drawn {
                draw(&system_table.boot_services());
                drawn = true;
            },
        };
    };
    Status::SUCCESS
}
