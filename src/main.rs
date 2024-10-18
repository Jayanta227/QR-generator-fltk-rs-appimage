use fltk::{app, button::Button, frame::Frame, image::RgbImage, input::Input, prelude::*, window::Window};
use qrcodegen::{QrCode, QrCodeEcc};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 400, 450, "QR Code Generator");

    // Input field for custom text
    let input = Input::new(50, 10, 300, 30, "");

    // Frame to display the QR code
    let frame = Frame::new(50, 50, 300, 300, "");

    // Button to generate QR code
    let mut button = Button::new(150, 360, 100, 30, "Generate QR");

    // Use Rc<RefCell<_>> to allow mutability
    let input_rc = Rc::new(RefCell::new(input));
    let frame_rc = Rc::new(RefCell::new(frame));

    // Function to generate and display the QR code
    let generate_qr_code = {
        let input_rc = Rc::clone(&input_rc);
        let frame_rc = Rc::clone(&frame_rc);
        move || {
            let input_text = input_rc.borrow().value();
            if input_text.is_empty() {
                println!("Please enter some text to generate a QR code.");
                return;
            }

            // Generate a QR code from the input text
            let qr_code = match QrCode::encode_text(&input_text, QrCodeEcc::Medium) {
                Ok(code) => code,
                Err(e) => {
                    println!("Error generating QR code: {}", e);
                    return;
                }
            };

            // Convert the QR code into pixel data
            let size = qr_code.size();
            let mut img_data = Vec::with_capacity((size * size * 3).try_into().unwrap());
            for y in 0..size {
                for x in 0..size {
                    let pixel = if qr_code.get_module(x, y) { 0 } else { 255 };
                    img_data.push(pixel);  // R
                    img_data.push(pixel);  // G
                    img_data.push(pixel);  // B
                }
            }

            // Convert the pixel data into an FLTK RgbImage
            let mut img = RgbImage::new(&img_data, size, size, fltk::enums::ColorDepth::Rgb8).unwrap();
            img.scale(300, 300, true, true);

            // Set the image in the frame
            frame_rc.borrow_mut().set_image(Some(img));
            frame_rc.borrow_mut().redraw();
        }
    };

    // Set button callback to generate the QR code
    let generate_qr_code_clone = generate_qr_code.clone();
    button.set_callback(move |_| {
        generate_qr_code_clone();
    });

    // Set the callback to listen for the "Enter" key press in the input field
    input_rc.borrow_mut().set_trigger(fltk::enums::CallbackTrigger::EnterKey);
    input_rc.borrow_mut().set_callback(move |_| {
        generate_qr_code();
    });

    win.end();
    win.show();
    app.run().unwrap();
}

