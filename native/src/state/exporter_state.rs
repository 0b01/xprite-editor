use crate::prelude::*;

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
    pub fn export(&self, stem: &str, layer: &Option<(usize, usize)>, xpr: &Xprite) {
        let ext = self.as_file_extension();
        let path = format!("{}.{}", stem, ext);

        if &ExporterFormat::ASE == self {
            xpr.save_ase(&path);
        } else {
            if let Some((group_idx, layer_idx)) = layer {
                xpr.save_layer_img(*group_idx, *layer_idx, &path);
            } else {
                xpr.save_img(&path);
            }
        }
    }
}

pub struct ExporterSpec {
    pub format: ExporterFormat,
    pub scale: f64,
    pub stem: String,
    pub layer: Option<(usize, usize)>,
}

impl Default for ExporterSpec {
    fn default() -> Self {
        Self {
            format: ExporterFormat::ASE,
            scale: 1.,
            stem: String::new(),
            layer: None,
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

    pub fn run_export(&self, xpr: &Xprite) {
        for s in &self.specs {
            let stem = &s.stem;
            let layer = &s.layer;
            s.format.export(&stem, layer, xpr);
        }
    }

}