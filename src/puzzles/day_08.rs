use crate::input::Input;
use colored::{ColoredString, Colorize};
use std::error::Error;

type Pixel = usize;
type Row = Vec<Pixel>;
type Layer = Vec<Row>;
type Image = Vec<Layer>;

fn read_layer(pixels: &[Pixel], width: usize, height: usize) -> Layer {
    pixels
        .chunks_exact(width)
        .map(|row| row.to_vec())
        .take(height)
        .collect::<Vec<_>>()
}

fn read_image(pixels: &[Pixel], width: usize, height: usize) -> Image {
    let mut image = Vec::new();

    let pixels_per_layer = width * height;

    let mut index = 0;
    while index < pixels.len() {
        let layer = read_layer(&pixels[index..], width, height);
        image.push(layer);
        index += pixels_per_layer;
    }

    image
}

fn parse_input(i: &Input) -> Option<Vec<Pixel>> {
    i.raw
        .trim()
        .chars()
        .map(|c| c.to_digit(10).map(|v| v as usize))
        .collect()
}

fn image_from_input(i: &Input, width: usize, height: usize) -> Option<Image> {
    parse_input(&i).map(|pixels| read_image(&pixels, width, height))
}

fn calc_first_part(i: &Input, width: usize, height: usize) -> Result<usize, Box<dyn Error>> {
    let image = image_from_input(i, width, height).ok_or("invalid image input")?;

    let mut lowest_zero = std::usize::MAX;
    let mut score = 0;

    for layer in image {
        let mut count0 = 0;
        let mut count1 = 0;
        let mut count2 = 0;

        for pixel in layer.iter().flatten() {
            match pixel {
                0 => count0 += 1,
                1 => count1 += 1,
                2 => count2 += 1,
                _ => (),
            }
        }

        if count0 < lowest_zero {
            lowest_zero = count0;
            score = count1 * count2;
        }
    }

    Ok(score)
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    calc_first_part(i, 25, 6).map(|v| v.to_string())
}

const COLOR_TRANSPARENT: usize = 2;

fn merge_layers(img: Image, width: usize, height: usize) -> Layer {
    let mut final_layer: Layer = Vec::with_capacity(height);

    for row_index in 0..height {
        let mut row = Vec::with_capacity(width);
        for width_index in 0..width {
            let mut color = COLOR_TRANSPARENT;
            for layer in img.iter() {
                color = layer[row_index][width_index];
                if color != COLOR_TRANSPARENT {
                    break;
                }
            }

            row.push(color);
        }

        final_layer.push(row);
    }

    final_layer
}

fn write_layer(writer: &mut impl std::fmt::Write, layer: Layer) -> Result<(), Box<dyn Error>> {
    for row in layer {
        for pixel in row {
            let mut text: ColoredString = pixel.to_string().as_str().into();
            match pixel {
                0 => text = text.on_black().dimmed(),
                1 => text = text.on_white(),
                _ => (),
            }

            write!(writer, "{}", text)?;
        }

        writer.write_char('\n')?;
    }

    Ok(())
}

fn calc_second(i: &Input, width: usize, height: usize) -> Result<String, Box<dyn Error>> {
    let image = image_from_input(i, width, height).ok_or("invalid image input")?;
    let layer = merge_layers(image, width, height);

    let mut out = String::from("\n");
    if let Err(e) = write_layer(&mut out, layer) {
        return Err(e);
    }

    Ok(out)
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    calc_second(i, 25, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(calc_first_part(&Input::new("123456789012"), 3, 2)?, 1);

        Ok(())
    }
}
