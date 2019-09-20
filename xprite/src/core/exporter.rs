use crate::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ExportType {
    All,
    Group(usize),
    Layer(usize, usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExporterFormat {
    ICO,
    JPG,
    PNG,
    PBM,
    PGM,
    PPM,
    PAM,
    BMP,
    ASE,
}

impl ExporterFormat {
    pub const VARIANTS: [ExporterFormat; 9] = [
        ExporterFormat::ICO,
        ExporterFormat::JPG,
        ExporterFormat::PNG,
        ExporterFormat::PBM,
        ExporterFormat::PGM,
        ExporterFormat::PPM,
        ExporterFormat::PAM,
        ExporterFormat::BMP,
        ExporterFormat::ASE,
    ];

    pub fn as_file_extension(self) -> &'static str {
        match &self {
            ExporterFormat::ICO => "ico",
            ExporterFormat::JPG => "jpg",
            ExporterFormat::PNG => "png",
            ExporterFormat::PBM => "pbm",
            ExporterFormat::PGM => "pgm",
            ExporterFormat::PPM => "ppm",
            ExporterFormat::PAM => "pam",
            ExporterFormat::BMP => "bmp",
            ExporterFormat::ASE => "ase",
        }
    }
}

pub struct ExporterSpec {
    pub format: ExporterFormat,
    pub rescale: u32,
    pub stem: String,
    pub layer: ExportType,
    pub trim: bool,
}

impl Default for ExporterSpec {
    fn default() -> Self {
        Self {
            format: ExporterFormat::ASE,
            rescale: 1,
            stem: String::new(),
            layer: ExportType::All,
            trim: true,
        }
    }
}

impl ExporterSpec {
    fn export(&self, xpr: &Xprite, dir: &str) {
        let ExporterSpec {
            format,
            rescale,
            stem,
            layer,
            trim,
        } = self;
        let ext = self.format.as_file_extension();

        let mut path = ::std::path::PathBuf::new();
        path.push(dir);
        path.set_file_name(if *rescale == 1 {
            stem.to_string()
        } else {
            format!("{}.{}x", stem, rescale)
        });
        path.set_extension(ext);

        match format {
            ExporterFormat::ASE => {
                xpr.save_ase(&path);
            }
            _ => {
                match layer {
                    ExportType::All => xpr.save_img(path, *rescale),
                    ExportType::Layer(group_idx, layer_idx) => xpr.save_layer_img(*group_idx, *layer_idx, &path, *rescale, *trim),
                    ExportType::Group(group_idx) => xpr.save_group_img(*group_idx, path, *rescale, *trim),
                };
            }
        }
    }
}

#[derive(Default)]
pub struct Exporter {
    pub specs: Vec<ExporterSpec>,
    pub selected: usize,
    pub path: String,
}

impl Exporter {
    pub fn add_default(&mut self) {
        self.specs.push(Default::default());
    }

    pub fn remove(&mut self, id: usize) {
        self.specs.remove(id);
    }

    pub fn set_scale(&mut self, id: usize, rescale: u32) {
        self.specs[id].rescale = rescale;
    }

    pub fn set_stem(&mut self, id: usize, stem: String) {
        self.specs[id].stem = stem;
    }

    pub fn set_format(&mut self, id: usize, fmt: ExporterFormat) {
        self.specs[id].format = fmt;
    }

    pub fn run_export(&self, xpr: &Xprite) {
        for s in &self.specs {
            s.export(xpr, &self.path);
        }
    }
}
