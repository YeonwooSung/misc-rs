use image::{self, imageops, GenericImageView};

fn main() {
    // set the resize image size
    let size = 128;

    // get the command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] simple-image-process-tool [image path]");
        return;
    }

    let infile = String::from(&args[1]);
    let file_name: Vec<&str> = infile.split('.').collect();
    let outfile = format!("{}_{}x{}.{}", file_name[0], size, size, file_name[1]);
    println!("infile: {}, outfile: {}", infile, outfile);

    // get the size of image
    let mut img = image::open(infile).expect("Failed to open image");
    let (width, height) = img.dimensions();

    // crop the image
    let mut img2 = imageops::crop(&mut img, width / 2, height / 2, width, height).to_image();
    let img3 = imageops::resize(&mut img2, size, size, imageops::FilterType::Lanczos3);

    // save the image
    img3.save(outfile).expect("Failed to save image");
}
