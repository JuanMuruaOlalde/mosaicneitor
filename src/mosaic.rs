pub struct Mosaic {
    pub base_image: image::Rgba32FImage,
    pub general_tessera_size: RectangleInMm,
    pub contents: Vec<Vec<Tessera>>,
}

#[derive(Clone)]
pub struct RectangleInMm {
    pub horizontal: usize,
    pub vertical: usize,
}
impl Copy for RectangleInMm {}

pub struct Tessera {
    pub color: palette::Oklch,
    //size: RectangleInMm,  to be implemented... (difficult... how to display different row sizes on the user interface ?!?)
    //shape: to be implemented... (even more difficult... how to represent a non-rectangular tessera of arbitrary shape ?!?)
}

impl Mosaic {
    pub fn new(base_image: image::Rgba32FImage, general_tessera_base_size: RectangleInMm) -> Self {
        Self {
            base_image,
            general_tessera_size: general_tessera_base_size,
            contents: Vec::new(),
        }
    }
    pub fn add_a_row_of_tesserae(&mut self, row: Vec<Tessera>) {
        self.contents.push(row);
    }

    pub fn get_number_of_rows(&self) -> usize {
        self.contents.len()
    }

    pub fn get_number_of_tesserae_in_row(&self, row_number: usize) -> usize {
        match self.contents.get(row_number) {
            Some(row) => row.len(),
            None => 0,
        }
    }
}
