use crate::vec3::Color;

fn main() {
    // Image dimensions
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // PPM header
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Render the image
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            // Create a gradient from red to green
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;  // Fixed blue component

            let pixel_color = Color::new(r, g, b);
            
            // Convert to 8-bit color values (0-255)
            let ir = (255.999 * pixel_color.x) as i32;
            let ig = (255.999 * pixel_color.y) as i32;
            let ib = (255.999 * pixel_color.z) as i32;

            // Print the pixel color
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.");
}
