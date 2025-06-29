#[derive(Debug, Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

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
            // Create a gradient from blue to white
            let t = (i + j) as f64 / ((IMAGE_WIDTH + IMAGE_HEIGHT) as f64 / 2.0);
            let r = t;  // Red increases with t
            let g = t;  // Green increases with t
            let b = 1.0;  // Full blue

            let pixel_color = Color::new(r, g, b);
            
            // Convert to 8-bit color values (0-255)
            let ir = (255.999 * pixel_color.r) as i32;
            let ig = (255.999 * pixel_color.g) as i32;
            let ib = (255.999 * pixel_color.b) as i32;

            // Print the pixel color
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.");
}
