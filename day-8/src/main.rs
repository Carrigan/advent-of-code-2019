use std::fmt;

struct Image {
    layers: Vec<Layer>,
    width: usize,
    height: usize
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.layers.len() {
            write!(f, "Layer {}\n{:?}\n\n", i, self.layers[i])?;
        }

        write!(f, "")
    }
}

impl Image {
    fn new(data: String, width: usize, height: usize) -> Self {
        let layer_size = width * height;
        let mut layers = Vec::new();
        let mut current = data;

        loop {
            if current.is_empty() { break }

            let remaining = current.split_off(layer_size);
            layers.push(Layer::new(current, width));
            current = remaining;
        }

        Image { layers, width, height }
    }

    fn fewest_zero_checksum(&self) -> usize {
        let first_layer = self.layers.first().unwrap();
        let mut zero_layer = first_layer;
        let mut zero_count = first_layer.number_of(0);

        for layer in &self.layers {
            let current_zeroes = layer.number_of(0);

            if current_zeroes < zero_count {
                zero_layer = layer;
                zero_count = current_zeroes;
            }
        }

        zero_layer.checksum()
    }

    fn decode(&self) -> Layer {
        let mut transparent_layer_string = String::new();
        for i in 0..(self.width * self.height) { transparent_layer_string.push('2') };
        let mut composite = Layer::new(transparent_layer_string, self.width);

        for layer in &self.layers {
            for i in 0..layer.pixels.len() {
                if composite.pixels[i] == 2 {
                    composite.pixels[i] = layer.pixels[i];
                }
            }
        }

        composite
    }
}

#[derive(Clone)]
struct Layer {
    width: usize,
    pixels: Vec<usize>
}

impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.pixels.len() / self.width;

        for row_i in 0..rows {
            for col_i in 0..self.width {
                write!(f, "{}", self.pixels[row_i * self.width + col_i])?;
            }

            write!(f, "\n")?;
        }

        write!(f, "")
    }
}

impl Layer {
    fn new(data: String, width: usize) -> Self {
        let pixels = data.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        Layer { pixels, width }
    }

    fn checksum(&self) -> usize {
        self.number_of(1) * self.number_of(2)
    }

    fn number_of(&self, value: usize) -> usize {
        let mut found = 0;

        for pixel in &self.pixels {
            if *pixel == value { found += 1 }
        }

        found
    }
}

fn main() {
    let image = Image::new(std::fs::read_to_string("input.txt").unwrap(), 25, 6);
    println!("{:?}", image.decode());
}

#[test]
fn test_image() {
    let image = Image::new(String::from("123456789012"), 3, 2);
    assert_eq!(image.fewest_zero_checksum(), 1);
}
