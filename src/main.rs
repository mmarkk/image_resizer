use image::imageops::FilterType;
use image::GenericImageView;
use std::env;
use std::path::Path;

fn main() {
    // Fetch the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the image file name argument is provided
    if args.len() < 2 {
        println!("Usage: cargo run <image_file>");
        return;
    }

    // Extract the image file name from the arguments
    let image_file = &args[1];

    // Load the image using the image crate
    let img = image::open(image_file).expect("Failed to open image");

    // Get the original image dimensions
    let (original_width, original_height) = img.dimensions();

    // Calculate the new dimensions while maintaining the aspect ratio
    let (new_width, new_height) = calculate_aspect_ratio(original_width, original_height, 800, 600);

    // Resize the image using a Lanczos3 filter
    let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);

    // Extract the original filename (without extension) from the path
    let file_stem = Path::new(image_file)
        .file_stem()
        .expect("Failed to extract file stem")
        .to_str()
        .expect("Failed to convert file stem to string");

    // Construct the new filename with the desired format
    let new_filename = format!("{}_800_600.jpg", file_stem);

    // Save the resized image to the new filename
    resized_img
        .save(&new_filename)
        .expect("Failed to save resized image");

    println!("Resized image saved as {}", new_filename);
}

// Function to calculate new dimensions while maintaining aspect ratio
fn calculate_aspect_ratio(original_width: u32, original_height: u32, max_width: u32, max_height: u32) -> (u32, u32) {
    let width_ratio = max_width as f32 / original_width as f32;
    let height_ratio = max_height as f32 / original_height as f32;

    if width_ratio < height_ratio {
        (max_width, (original_height as f32 * width_ratio) as u32)
    } else {
        ((original_width as f32 * height_ratio) as u32, max_height)
    }
}
