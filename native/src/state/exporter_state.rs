#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExporterFormat {
    ICO,
    JPEG,
    PNG,
    PBM,
    PGM,
    PPM,
    PAM,
    BMP,
    ASE,
}

impl ExporterFormat {
    pub const VARIANTS: [ExporterFormat;9] = [
        ExporterFormat::ICO,
        ExporterFormat::JPEG,
        ExporterFormat::PNG,
        ExporterFormat::PBM,
        ExporterFormat::PGM,
        ExporterFormat::PPM,
        ExporterFormat::PAM,
        ExporterFormat::BMP,
        ExporterFormat::ASE,
    ];

    pub fn to_file_extension(&self) -> &'static str {
        match &self {
            ICO => "ico",
            JPEG => "jpg",
            PNG => "png",
            PBM => "pbm",
            PGM => "pgm",
            PPM => "ppm",
            PAM => "pam",
            BMP => "bmp",
            ASE => "ase",
        }
    }
}

pub struct ExporterSpec {
    pub format: ExporterFormat,
    pub scale: f64,
}

impl Default for ExporterSpec {
    fn default() -> Self {
        Self {
            format: ExporterFormat::ASE,
            scale: 1.,
        }
    }
}

#[derive(Default)]
pub struct ExporterState {
    pub show: bool,
    pub specs: Vec<ExporterSpec>,
    pub selected: usize,
    pub path: String,
}

impl ExporterState {

    pub fn add_default(&mut self) {
        self.specs.push(Default::default());
    }

    pub fn remove(&mut self, id: usize) {
        self.specs.remove(id);
    }

    pub fn set_scale(&mut self, id: usize, scale: f64) {
        self.specs[id].scale = scale;
    }

    pub fn set_format(&mut self, id: usize, fmt: ExporterFormat) {
        self.specs[id].format = fmt;
    }

}