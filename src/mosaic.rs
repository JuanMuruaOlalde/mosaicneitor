pub struct Mosaic {
    base_image: Option<image::Rgba32FImage>,
    general_tessera_size: RectangleInMm,
    contents: Vec<Vec<Tessera>>,
}

#[derive(Debug)]
pub struct Tessera {
    pub color: palette::Oklch,
    //size: RectangleInMm,  to be implemented... (difficult... how to display different row sizes on the user interface ?!?)
    //shape: to be implemented... (even more difficult... how to represent a non-rectangular tessera of arbitrary shape ?!?)
}

#[derive(Clone)]
pub struct RectangleInMm {
    pub horizontal: usize,
    pub vertical: usize,
}
impl Copy for RectangleInMm {}

#[derive(PartialEq, Debug)]
pub struct PositionOnGrid {
    pub row: usize,
    pub column: usize,
}

impl Mosaic {
    pub fn new(
        base_image: Option<image::Rgba32FImage>,
        general_tessera_base_size: RectangleInMm,
    ) -> Self {
        Self {
            base_image,
            general_tessera_size: general_tessera_base_size,
            contents: Vec::new(),
        }
    }

    pub fn get_general_tessera_size(&self) -> &RectangleInMm {
        &self.general_tessera_size
    }

    pub fn get_contents(&self) -> &Vec<Vec<Tessera>> {
        &self.contents
    }

    pub fn add_a_row_of_tesserae(&mut self, row: Vec<Tessera>) {
        self.contents.push(row);
    }

    pub fn change_tessera(
        &mut self,
        position: &PositionOnGrid,
        new_tessera: Tessera,
    ) -> Result<(), String> {
        if position.row > self.contents.len() {
            return Err(format!(
                "Out of bounds! The mosaic has only {} rows. And you want to change the {}nt row.",
                self.contents.len(),
                position.row
            ));
        }
        if position.column > self.contents[position.row].len() {
            return Err(format!("Out of bounds! The {}nt row in the mosaic has only {} columns. And you want to change the {}nt column.", position.row, self.contents[position.row].len(), position.column));
        }
        let _ = std::mem::replace(
            &mut self.contents[position.row - 1][position.column - 1],
            new_tessera,
        );
        Ok(())
    }

    // this methods is used only for tests
    pub fn get_number_of_rows(&self) -> usize {
        self.contents.len()
    }

    // this methods is used only for tests
    pub fn get_number_of_tesserae_in_row(&self, row_number: usize) -> usize {
        match self.contents.get(row_number) {
            Some(row) => row.len(),
            None => 0,
        }
    }
}
