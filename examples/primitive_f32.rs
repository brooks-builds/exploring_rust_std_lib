#![allow(
    clippy::float_cmp,
    clippy::eq_op,
    clippy::cmp_nan,
    clippy::approx_constant,
    dead_code,
    clippy::assertions_on_constants,
    clippy::precedence
)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn length(&self) -> f32 {
        self.x.hypot(self.y)
    }
}

fn main() {
    assert_eq!(0.1 + 0.2, 0.30000000000000004);
    assert_eq!(-0.0, 0.0);
    assert_eq!(1.0 / 0.0, f32::INFINITY);
    assert!(f32::NAN != f32::NAN);
    assert!(!15.0_f32.is_nan());
    implementations();
}

fn implementations() {
    assert!(10.5_f32.floor() == 10.0);
    assert!(10.5_f32.ceil() == 11.0);
    assert!(10.5_f32.round() == 11.0);
    assert!(10.4_f32.round() == 10.0);
    assert!(3.14_f32.trunc() == 3.0);
    assert_eq!(3.14_f32.fract(), 0.1400001);
    assert_eq!(-15.0_f32.abs(), -15.0);
    assert_eq!((-15.0_f32).abs(), 15.0);
    assert_eq!(-15.0_f32.signum(), -1.0);
    assert_eq!((-15.0_f32).copysign(1.0), 15.0);
    assert_eq!(10.2_f32.mul_add(2.3, 5.5), 28.96);
    assert_eq!(10.2_f32 * 2.3 + 5.5, 28.96);
    assert_eq!(10.0_f32.div_euclid(5.0), 2.0);
    assert_eq!(10.0_f32.div_euclid(6.0), 1.0);
    assert_eq!(10.4_f32.div_euclid(6.2), 1.0);
    assert_eq!(10.0_f32.rem_euclid(6.0), 10.0_f32 % 6.0);
    assert_eq!(2.0_f32.powi(4), 16.0);
    assert_eq!(2.2_f32.powi(4), 23.425602);
    assert_eq!(2.2_f32.powf(4.3), 29.676838);
    assert_eq!(152.2_f32.sqrt(), 12.336936);
    assert_eq!(1.0_f32.exp(), 2.7182817);
    assert_eq!(1.0_f32.exp2(), 2.0);
    assert_eq!(100.0_f32.ln(), 4.6051702);
    assert_eq!(100.0_f32.log(10.0), 2.0);
    assert_eq!(100.0_f32.log2(), 6.643856);
    assert_eq!(100.0_f32.log10(), 2.0);
    assert_eq!(100.0_f32.cbrt(), 4.6415887);
    assert_eq!(5.0_f32.hypot(10.0), 11.18034);
    assert_eq!(45.0_f32.sin(), 0.8509035);
    assert_eq!(45.0_f32.cos(), 0.52532196);
    assert_eq!(45.0_f32.cos(), 0.52532196);
    assert_eq!(45.0_f32.tan(), 1.6197752);
}
