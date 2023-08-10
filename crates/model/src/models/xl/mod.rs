pub struct Xl {
    pub workbook: String,
    pub styles: String,
    pub shared_strings: String,
    pub worksheets: Vec<Worksheet>,
    pub themes: Vec<Theme>,
    pub rels: WorkbookRels,
}

pub struct Worksheet {}

pub struct Theme {}

pub struct WorkbookRels {}
