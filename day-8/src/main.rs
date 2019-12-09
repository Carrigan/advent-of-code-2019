#[derive(Debug)]
struct Image {
    layers: Vec<Layer>
}

impl Image {
    fn new(data: String, width: usize, height: usize) -> Self {
        let layer_size = width * height;
        let mut layers = Vec::new();
        let mut current = data;

        loop {
            if current.is_empty() { break }

            let remaining = current.split_off(layer_size);
            layers.push(Layer::new(current));
            current = remaining;
        }

        Image { layers }
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
}

#[derive(Debug)]
struct Layer {
    pixels: Vec<usize>
}

impl Layer {
    fn new(data: String) -> Self {
        let pixels = data.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        Layer { pixels }
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
    println!("Lowest checksum: {:?}", image.fewest_zero_checksum());
}

#[test]
fn test_image() {
    let image = Image::new(String::from("123456789012"), 3, 2);
    assert_eq!(image.fewest_zero_checksum(), 1);
}
