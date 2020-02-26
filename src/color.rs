use std::{convert::From};

#[derive(Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Clone)]
pub struct LAB {
    pub l: f32,
    pub a: f32,
    pub b: f32
}

#[derive(Clone)]
pub struct HSL {
    pub h: f32,
    pub s: f32,
    pub l: f32
}

// RGB -> XYZ -> LAB conversions and vice versa from https://www.easyrgb.com/en/math.php
// Continuity correction of the function from http://www.brucelindbloom.com/index.html?LContinuity.html
const KAPPA: f32 = 24389.0 / 27.0;
const EPSILON: f32 = 216.0 / 24389.0;
const EPSILON_CUBE_ROOT: f32 = 0.20689655172413796;

fn map_rgb_xyz(val: f32) -> f32 {
    return (val / 255.0).powf(2.19921875) * 100.0;
}

fn map_xyz_rgb(val: f32) -> u8 {
    (val.powf(1.0 / 2.19921875) * 255.0) as u8
}


fn map_xyz_lab(val: f32) -> f32 {
    if val > EPSILON {
        return val.powf(1.0 / 3.0);
    } else {
        return (KAPPA * val + 16.0) / 116.0;
    }
}

impl RGB {
    /**
     * Converts the color to the corresponding hex color code
     */
    pub fn hex(&self) -> String {
        return str::replace(
            &format!("#{:2X}{:2X}{:2X}", self.r, self.g, self.b),
            " ",
            "0",
        );
    }

    /**
     * Converts the color to the corresponding XYZ color space
     */
    pub fn to_xyz(&self) -> (f32, f32, f32) {
        let var_r = map_rgb_xyz(self.r.into());
        let var_g = map_rgb_xyz(self.g.into());
        let var_b = map_rgb_xyz(self.b.into());

        return (
            var_r*0.57667 + var_g*0.18555 + var_b*0.18819,
            var_r*0.29738 + var_g*0.62735 + var_b*0.07527,
            var_r*0.02703 + var_g*0.07069 + var_b*0.99110
        );
    }
}

impl From<&LAB> for RGB {

    /**
     * Creates equivalent RGB color from LAB color
     */
    fn from(color: &LAB) -> Self {
        let xyz = color.to_xyz();
        let var_x = xyz.0 / 100.0;
        let var_y = xyz.1 / 100.0;
        let var_z = xyz.2 / 100.0;
        
        return RGB {
            r: map_xyz_rgb(var_x*2.04137 + var_y*-0.56495 + var_z*-0.34469),
            g: map_xyz_rgb(var_x*-0.96927 + var_y*1.87601 + var_z*0.04156),
            b: map_xyz_rgb(var_x*0.01345 + var_y*-0.11839 + var_z*1.01541)
        };
    }
}

fn hue_to_rgb(v1: f32, v2: f32, mut vh: f32) -> f32 {
    if vh < 0.0 {
        vh += 1.0;
    }
    if vh > 1.0 {
        vh -= 1.0;
    }

    if 6.0 * vh < 1.0 {
        return v1 + ( v2 - v1 ) * 6.0 * vh;
    }
    if 2.0 * vh < 1.0 {
        return v2;
    }
    if 3.0 * vh < 2.0 {
        return v1 + ( v2 - v1 ) * ( ( 2.0 / 3.0 ) - vh ) * 6.0;
    }

    return v1;
}

impl From<&HSL> for RGB {
    fn from(color: &HSL) -> Self {
        let mut rgb_color = RGB {
            r: (color.l * 255.0) as u8,
            g: (color.l * 255.0) as u8,
            b: (color.l * 255.0) as u8
        };

        if color.s != 0.0 {
            let var_2 = if color.l < 0.5 {
                color.l * (1.0 + color.s)
            } else {
                (color.l + color.s) - (color.s * color.l)
            };

            let var_1 = 2.0 * color.l - var_2;

            rgb_color.r = (255.0 * hue_to_rgb(var_1, var_2, color.h + 1.0/3.0)) as u8;
            rgb_color.g = (255.0 * hue_to_rgb(var_1, var_2, color.h)) as u8;
            rgb_color.b = (255.0 * hue_to_rgb(var_1, var_2, color.h - ( 1.0 / 3.0 ) )) as u8;
        }

        return rgb_color;
    }
}


impl LAB {

    /**
     * Calculates the chroma of the color 
     */
    pub fn chroma(&self) -> f32 {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    /**
     * Finds the index and distance from nearest color from a group of colors
     */
    pub fn nearest(&self, colors: &Vec<LAB>) -> (usize, f32) {
        return colors
            .iter()
            .map(|c| self.distance(c))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(&b).expect("NaN encountered"))
            .unwrap();
    }

    /**
     * Calculates Delta E(1994) between two colors
     */
    pub fn distance(&self, color: &LAB) -> f32 {

        let xc1 = (self.a.powi(2) + self.b.powi(2)).sqrt();
        let xc2 = (color.a.powi(2) + color.b.powi(2)).sqrt();
        let xdl = color.l - self.l;
        let mut xdc = xc2 - xc1;
        let xde = ( (self.l - color.l).powi(2) + (self.a - color.a).powi(2) + (self.b - color.b).powi(2) ).sqrt();

        let mut xdh = xde.powi(2) - xdl.powi(2) - xdc.powi(2);
        if xdh > 0.0 {
            xdh = xdh.sqrt();
        } else {
            xdh = 0.0;
        }

        let xsc = 1.0 + 0.045 * xc1;
        let xsh = 1.0 + 0.015 * xc1;
        xdc /= xsc;
        xdh /= xsh;

        return ( xdl.powi(2) + xdc.powi(2) + xdh.powi(2) ).sqrt();
    }

    pub fn to_xyz(&self) -> (f32, f32, f32) {
        let mut var_y = (self.l + 16.0) / 116.0;
        let mut var_x = self.a / 500.0 + var_y;
        let mut var_z = var_y - self.b / 200.0;

        if var_x > EPSILON_CUBE_ROOT {
            var_x = var_x.powi(3);
        } else {
            var_x = ((var_x * 116.0) - 16.0) / KAPPA;
        }
        if self.l > EPSILON * KAPPA {
            var_y = var_y.powi(3);
        } else {
            var_y = self.l / KAPPA;
        }
        if var_z > EPSILON_CUBE_ROOT {
            var_z = var_z.powi(3);
        } else {
            var_z = ((var_z * 116.0) - 16.0) / KAPPA;
        }

        return (var_x * 95.047, var_y * 100.0, var_z * 108.883);
    }

}

impl From<&RGB> for LAB {

    /**
     * Creates equivalent LAB color from RGB color
     */
    fn from(color: &RGB) -> Self {
        let xyz = color.to_xyz();

        let var_x = map_xyz_lab(xyz.0 / 95.047);
        let var_y = map_xyz_lab(xyz.1 / 100.0);
        let var_z = map_xyz_lab(xyz.2 / 108.883);

        return LAB {
            l: 116.0 * var_y - 16.0,
            a: 500.0 * (var_x - var_y),
            b: 200.0 * (var_y - var_z)
        };
    }
}

impl From<&RGB> for HSL {
    fn from(color: &RGB) -> Self {
        let var_r = color.r as f32 / 255.0;
        let var_g = color.g as f32 / 255.0;
        let var_b = color.b as f32 / 255.0;

        let var_max = var_r.max(var_g.max(var_b));
        let var_min = var_r.min(var_g.min(var_b));
        let del_max = var_max - var_min;

        let mut hsl_color = HSL {
            h: 0.0,
            s: 0.0,
            l: (var_max + var_min) / 2.0
        };

        if del_max != 0.0 {
            hsl_color.s = if hsl_color.l < 0.5 {
                del_max / (var_max + var_min)
            } else {
                del_max / (2.0 - var_max - var_min)
            };

            let del_r = (((var_max - var_r) / 6.0) + del_max / 2.0) / del_max;
            let del_g = (((var_max - var_g) / 6.0) + del_max / 2.0) / del_max;
            let del_b = (((var_max - var_b) / 6.0) + del_max / 2.0) / del_max;

            hsl_color.h = if var_r == var_max {
                del_b - del_g
            } else if var_g == var_max {
                1.0 / 3.0 + del_r - del_b
            } else if var_b == var_max {
                2.0 / 3.0 + del_g - del_r
            } else {
                hsl_color.h
            };

            if hsl_color.h < 0.0 {
                hsl_color.h += 1.0;
            } else if hsl_color.h > 1.0 {
                hsl_color.h -= 1.0;
            }
        }

        return hsl_color;
    }
}

impl From<&LAB> for HSL {
    fn from(color: &LAB) -> Self {
        return HSL::from(&RGB::from(color));
    }
}

impl PartialEq for LAB {
    fn eq(&self, other: &Self) -> bool {
        return !(self.l != other.l || self.a != other.a || self.b != other.b);
    }
}

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        return !(self.r != other.r || self.g != other.g || self.b != other.b);
    }
}