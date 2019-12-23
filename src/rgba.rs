use std::{ops, convert::From};

#[derive(Clone)]
pub struct RGBA<T=u8> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}
pub type Pixels = Vec<RGBA>;

impl RGBA {
    pub fn brightness(&self) -> f32 {
        return 0.2126*(self.r as f32) + 0.7152*(self.g as f32) + 0.0722*(self.b as f32);
    }

    /**
     * Finds the index and distance from nearest color from a group of colors
     */
    pub fn nearest(&self, colors: &Pixels) -> (usize, f32) {
        return colors
            .iter()
            .map(|c| self.distance(c))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(&b).expect("NaN encountered"))
            .unwrap();
    }

    /**
     * Calculates Eucledian distance between two colors
     */
    pub fn distance(&self, color: &RGBA) -> f32 {
        let diff = RGBA::<i32>::from(self.clone()) - RGBA::<i32>::from(color.clone());

        return ((diff.r.pow(2) + diff.g.pow(2) + diff.b.pow(2) + diff.a.pow(2)) as f32).sqrt();
    }

    /**
     * Converts the color to the corresponding hex color code
     */
    pub fn hex(&self) -> String {
        str::replace(
            &format!("#{:2X}{:2X}{:2X}{:2X}", self.r, self.g, self.b, self.a),
            " ",
            "0",
        )
    }
}

impl From<RGBA<i32>> for RGBA<u8> {
    fn from(item: RGBA<i32>) -> Self {
        RGBA {
            r: item.r as u8,
            g: item.g as u8,
            b: item.b as u8,
            a: item.a as u8
        }
    }
}

impl From<RGBA<u8>> for RGBA<i32> {
    fn from(item: RGBA<u8>) -> Self {
        RGBA {
            r: item.r as i32,
            g: item.g as i32,
            b: item.b as i32,
            a: item.a as i32
        }
    }
}

impl ops::Sub for RGBA<i32> {
    type Output = RGBA<i32>;

    fn sub(self, _rhs: Self) -> Self::Output {
        let lhs: RGBA<i32> = self.into();
        let rhs: RGBA<i32> = _rhs.into();

        return RGBA::<i32> {
            r: lhs.r - rhs.r,
            g: lhs.g - rhs.g,
            b: lhs.b - rhs.b,
            a: lhs.a - rhs.a
        }
    }
}

impl PartialEq for RGBA {
    fn eq(&self, other: &Self) -> bool {
        return !(self.r != other.r || self.g != other.g || self.b != other.b || self.a != other.a);
    }
}
