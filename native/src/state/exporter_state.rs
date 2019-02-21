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
    pub const VARIANTS: [ExporterFormat;9] = [
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

    pub fn as_file_extension(&self) -> &'static str {
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

    /// export format to path
    pub fn export(&self, stem: &str, rescale: u32, layer: ExportType, xpr: &Xprite) {
        let ext = self.as_file_extension();
        let path = if rescale == 1 {
            format!("{}.{}", stem, ext)
        } else {
            format!("{}.{}x.{}", stem, rescale, ext)
        };

        match self {
            ExporterFormat::ASE => {
                xpr.save_ase(&path);
            }
            _ => {
                match layer {
                    ExportType::All => {xpr.save_img(&path, rescale)}
                    ExportType::Layer(group_idx, layer_idx) =>
                        {xpr.save_layer_img(group_idx, layer_idx, &path, rescale)}
                    ExportType::Group(group_idx) => {
                        {xpr.save_group_img(group_idx, &path, rescale)}
                    }
                };
            }
        };
    }
}

pub struct ExporterSpec {
    pub format: ExporterFormat,
    pub rescale: u32,
    pub stem: String,
    pub layer: ExportType,
}

impl Default for ExporterSpec {
    fn default() -> Self {
        Self {
            format: ExporterFormat::ASE,
            rescale: 1,
            stem: String::new(),
            layer: ExportType::All,
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
            let stem = &s.stem;
            let rescale = s.rescale;
            let layer = &s.layer;
            s.format.export(&stem, rescale, *layer, xpr);
        }
    }

}