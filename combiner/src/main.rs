mod args;
use args::Args;
use image::{ io::Reader, DynamicImage, ImageFormat, GenericImageView, imageops::FilterType::Triangle };
use std::{ io::BufReader, fs::File };

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferToSmall,
}

struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity: u32 = width * height * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage { width, height, data: buffer, name }
    }
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferToSmall)
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (_image_1, image_format_1) = find_image_from_path(args.image_1);
    let (_image_2, image_format_2) = find_image_from_path(args.image_2);

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    
    let (image_1, image_2) = standardise_size(_image_1, _image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data = combine_image(image_1, image_2);
    output.set_data(combined_data)?;
    image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image_format_1).unwrap();
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dimentions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 {dim_1} else {dim_2}
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimentions(image_1.dimensions(), image_2.dimensions());
    println!("Width: {} | Height: {}", width, height);

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

fn combine_image(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();
    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i+3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i+3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }
    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds")
        };
        rgba.push(val)
    }
    rgba
}