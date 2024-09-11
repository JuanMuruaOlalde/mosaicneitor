use crate::config;

pub struct Mosaic {
    tessela_base_size: SizeInMm,
    how_many_columns: usize,
    how_many_rows: usize,
    grid: Vec<Vec<Tessela>>,
}

pub struct SizeInMm {
    horizontal: usize,
    vertical: usize,
}

pub struct Tessela {
    color: palette::Oklab,
    //shape: to be implemented... (how to represent a non-rectangular tessela of arbitrary shape ?!?)
}

impl Mosaic {
    pub fn new(how_many_columns: usize, how_many_rows: usize) -> Self {
        Self {
            tessela_base_size: SizeInMm {
                horizontal: config::DEFAULT_BASE_TESSELA_SIZE_HORIZONTAL_MM,
                vertical: config::DEFAULT_BASE_TESSELA_SIZE_VERTICAL_MM,
            },
            how_many_columns: how_many_columns,
            how_many_rows: how_many_rows,
            grid: Vec::new(),
        }
    }
}
