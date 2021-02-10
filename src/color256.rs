use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }

    pub fn distance_to(&self, other: &Rgb) -> f32 {
        let dr = (other.r - self.r) as f32;
        let dg = (other.g - self.g) as f32;
        let db = (other.b - self.b) as f32;
        (dr * dr + dg * dg + db * db).sqrt()
    }

    pub fn brightness(&self) -> u8 {
        let Self {r, g, b} = *self;
        ((r as u16 + g as u16 + b as u16) / 3) as u8
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Color {
    #[serde(rename = "colorId")]
    pub id: u8,
    pub rgb: Rgb,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Color256 {
    pub colors: Vec<Color>,
}

impl Color256 {
    pub fn new(json_colors: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json_colors)?)
    }

    pub fn approx_from_rgb(&self, rgb: &Rgb) -> &Color {
        self.colors.iter().fold(&self.colors[0], |acc, e| {
            if rgb.distance_to(&e.rgb) < rgb.distance_to(&acc.rgb) {
                e
            } else {
                acc
            }
        })
    }
}
