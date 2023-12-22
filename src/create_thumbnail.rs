use clap::Parser;
use image::io::Reader as ImageReader;

use crate::console;

pub fn create_thumbnail() {
    let args = console::ThumbNailArgs::parse();

    // read image file
    let img = ImageReader::open(&args.image);

    match img {
        Ok(img) => {
            let thumbnail = img.decode();
            match thumbnail {
                Ok(thumbnail) => {
                    let thumbnail = thumbnail.thumbnail(50, 50);
                    // save the thumbnail
                    let save = thumbnail.save("thumbnail.png");
                    match save {
                        Ok(_) => {
                            println!("Thumbnail created successfully");
                        }
                        Err(err) => {
                            println!("Error creating thumbnail: {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("You have given the wrong image format !!! because {}", err);
                }
            }
        }
        Err(err) => {
            println!("Error reading image: {}", err);
        }
    }

}
