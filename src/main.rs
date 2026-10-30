use image::image_dimensions;
use image::{codecs::jpeg::JpegEncoder, imageops::FilterType};
use rfd::FileDialog;
use std::fs::File;
use std::io::{self, Write};

// Helper function to read a number from the terminal
// simple!
fn prompt_for_number(prompt: &str, default: u32) -> u32 {
    let mut input = String::new();
    print!("{} [default/Current {}]: ", prompt, default);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();

    if trimmed.is_empty() {
        default
    } else {
        trimmed.parse().unwrap_or(default)
    }
}

fn main() {
    println!("Impress will compress the pictures to the limit.\n");
    loop {
        println!("Select an image to resize...");

        // 1. Open a native file browser to pick the input image
        let input_path = match FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "heic", "bmp"])
            .pick_file()
        {
            Some(path) => path,
            None => {
                println!("No file selected. Try again.");
                continue;
            }
        };

        println!("Selected: {}", input_path.display());

        //changes start here:
        //hold the current size of the image and set that to the default!
        //let picture = image::open(input_path).unwrap();
        let (default_width, default_height) = image_dimensions(&input_path).unwrap();

        // 2. Prompt the user for dimensions and quality in the terminal
        let width = prompt_for_number("Enter target width", default_width);
        let height = prompt_for_number("Enter target height", default_height);
        let quality = prompt_for_number("Enter JPEG quality (1-100%)", 80) as u8;

        println!("Step 2: Choose where to save the new image...");

        // 3. Open a native file browser to choose the save location
        let output_path = match FileDialog::new()
            .set_file_name("resized_image.jpg")
            .add_filter("JPEG Image", &["jpg", "jpeg"])
            .add_filter("PNG Image", &["png"])
            .save_file()
        {
            Some(path) => path,
            None => {
                println!("No save location selected.\n  please try again.");
                continue;
            }
        };

        println!("Processing Image...");

        // 4. Load, resize, and compress
        let img = match image::open(&input_path) {
            Ok(image) => image,
            Err(e) => {
                eprintln!("Failed to open image: {}", e);
                continue;
            }
        };

        let resized_img = img.resize_exact(width, height, FilterType::Nearest);

        let mut out_file = match File::create(&output_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to create output file: {}", e);
                println!("Try again?");
                continue;
            }
        };

        let ext = output_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        if ext == "jpg" || ext == "jpeg" {
            let mut encoder = JpegEncoder::new_with_quality(&mut out_file, quality);
            if let Err(e) = encoder.encode_image(&resized_img) {
                eprintln!("Failed to encode JPEG: {}", e);
            } else {
                println!("✅ Success! Saved to {}", output_path.display());
            }
        } else {
            if let Err(e) = resized_img.write_to(
                &mut out_file,
                image::ImageFormat::from_path(&output_path).unwrap(),
            ) {
                eprintln!("Failed to save image: {}", e);
            } else {
                println!("✅ Success! Saved to {}", output_path.display());
            }
        }
        //naughty line I added with elementary levels of coding.
        println!("type Y to continue");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read...");
        //further interact with the choice, it must be a char
        // This is cool because it grabs the vary first char of the string, so the string Yes and yes also work
        // so the total yes answers are y, Y, yes, and Yes
        let choice: char = match choice.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };
        match choice {
            'Y' => continue,
            'y' => continue,
            _ => {
                println!("Will not continue || Exiting...");
                return;
            }
        }
    }
}
