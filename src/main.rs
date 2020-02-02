fn main() {
    let (r, g, b) = (
        rgb_limited_range(16),
        rgb_limited_range(16),
        rgb_limited_range(16),
    );

    dbg!(r);
    dbg!(g);
    dbg!(b);

    let (y, cb, cr) = ycbcr_bt709(r, g, b);
    dbg!(y);
    dbg!(cb);
    dbg!(cr);
}

pub fn ycbcr_bt709(r: f32, g: f32, b: f32) -> (u8, u8, u8) {
    // BT.601 conversion formula, with BT.709 parameters (kb, kr, kg)
    // https://en.wikipedia.org/wiki/YCbCr#ITU-R_BT.601_conversion
    
    let kb = 0.0722;
    let kr = 0.2126;
    let kg = 1.0 - kb - kr;

    let yp = kr * r + kg * g + kb * b;
    let pb = 0.5 * ((b - yp) / (1.0 - kb));
    let pr = 0.5 * ((r - yp) / (1.0 - kr));

    let y = 16_f32 + (219_f32 * yp);
    let cb = 128_f32 + (224_f32 * pb);
    let cr = 128_f32 + (224_f32 * pr);
    (y.round() as u8, cb.round() as u8, cr.round() as u8)
}

pub fn rgb_limited_range(value: u8) -> f32 {
    let steps: u8 = 235 - 16;
    (value - 16u8) as f32 / steps as f32
}

pub fn srgb_gamma_encode(value: f32) -> f32 {
    if value <= 0.0031308 {
        12.92 * value
    } else {
        (1.055 * value.powf(1.0 / 2.4)) - 0.055
    }
}

pub fn srgb_gamma_decode(value: f32) -> f32 {
    if value <= 0.04045 {
        value * 12.92_f32.recip()
    } else {
        ((value + 0.055) * 1.055_f32.recip()).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! color_test {
        ($name:ident, ($r:literal, $g:literal, $b:literal), ($y:literal, $cb:literal, $cr:literal)) => {
            #[test]
            fn $name() {
                let (input, expected) = (($r, $g, $b), ($y, $cb, $cr));
                let (r, g, b) = (
                    rgb_limited_range(input.0),
                    rgb_limited_range(input.1),
                    rgb_limited_range(input.2),
                );
                let (y, cb, cr) = ycbcr_bt709(r, g, b);
                assert_eq!(y, expected.0);
                assert_eq!(cb, expected.1);
                assert_eq!(cr, expected.2);
            }
        };
    }
    
    color_test!(white, (235, 235, 235), (235, 128, 128));
    color_test!(gray80, (213, 213, 213), (213, 128, 128));
    color_test!(gray65, (196, 196, 196), (196, 128, 128));
    color_test!(gray50, (176, 176, 176), (176, 128, 128));
    color_test!(gray35, (152, 152, 152), (152, 128, 128));
    color_test!(black, (16, 16, 16), (16, 128, 128));
    color_test!(dark_skin, (115, 86, 73), (91, 118, 143));
    color_test!(light_skin, (182, 145, 128), (152, 115, 148));
    color_test!(blue_sky, (97, 121, 150), (118, 146, 114));
    color_test!(foliage, (93, 108, 73), (102, 112, 122));
    color_test!(blue_flower, (128, 126, 167), (129, 149, 127));
    color_test!(blueish_green, (101, 178, 161), (160, 128, 89));
    color_test!(orange, (202, 119, 51), (132, 83, 174));
    color_test!(purplish_blue, (80, 95, 156), (96, 161, 117));
    color_test!(moderate_red, (182, 88, 99), (109, 123, 176));
    color_test!(purple, (95, 69, 108), (77, 145, 139));
    color_test!(yellow_green, (152, 176, 71), (163, 77, 121));
    color_test!(orange_yellow, (213, 154, 55), (159, 70, 163));
    color_test!(blue, (60, 69, 145), (73, 168, 120));
    color_test!(green, (77, 143, 77), (124, 102, 97));
    color_test!(red, (167, 58, 66), (82, 119, 183));
    color_test!(yellow, (220, 187, 44), (184, 51, 152));
    color_test!(magenta, (176, 88, 141), (111, 145, 171));
    color_test!(cyan, (16, 130, 156), (108, 155, 68));
    color_test!(red100, (235, 16, 16), (63, 102, 240));
    color_test!(green100, (16, 235, 16), (173, 42, 26));
    color_test!(blue100, (16, 16, 235), (32, 240, 118));
    color_test!(cyan100, (16, 235, 235), (188, 154, 16));
    color_test!(magenta100, (235, 16, 235), (78, 214, 230));
    color_test!(yellow100, (235, 235, 16), (219, 16, 138));
}
