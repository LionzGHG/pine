use std::ops::{Add, Mul, Sub};

use crate::util::Floatify;

/// ## Description
/// Represents a 2D vector in floating-point space.
/// 
/// `Vec2` is used extensively throughout the engine for:
/// - Positions
/// - Velocities
/// - Sizes
/// - Directions
/// - Physics calculations
/// 
/// Supports common arithmetic operations such as:
/// - Addition
/// - Subtraction
/// - Scalar multiplication
/// 
/// - **Item-Type**: Mathematical Primitive
/// 
/// ## Example
/// ```
/// let a = Vec2::new(10, 20);
/// let b = Vec2::new(5.0, 3.0);
/// let result = a + b;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Sub<Self> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Add<Self> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

/// ## Description
/// Creates a new [`Vec2`] from two numeric values.
///
/// Accepts any type implementing [Floatify], allowing both
/// integers and floating-point values to be passed directly.
///
/// This improves ergonomics when mixing [`i32`] and [`f32`]
/// values in gameplay code.
///
/// ## Example
/// ```
/// let v1 = vec2![10, 20.5];   // Vec2::new(10, 20.5)
/// let v2 = vec2![10];         // Vec2::new(10, 10)
/// let v3 = vec2![];           // Vec2::new(0, 0)
/// ```
/// ## Technical Info
/// Macro `vec2` expands to:
/// ```
/// Vec2::new($x, $x);
/// ```
#[macro_export]
macro_rules! vec2 {
    [$x:expr, $y:expr] => {
        Vec2::new($x, $y)
    };
    [$x:expr] => {
        Vec2::new($x, $x)
    };
    [] => {
        Vec2::new(0, 0)
    };
}

/*
#[inline(always)]
pub fn vec2(x: impl Floatify, y: impl Floatify) -> Vec2 {
    Vec2::new(x, y)
}
*/

impl Vec2 {

    pub const ZERO: Vec2 = Vec2::new_f32(0., 0.);

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline(always)]
    pub const fn new_f32(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    /// ## Description
    /// Creates a new [`Vec2`] from numeric values.
    ///
    /// Accepts any type implementing [Floatify], allowing both
    /// integers and floating-point values to be passed directly.
    ///
    /// This improves ergonomics when mixing [`i32`] and [`f32`]
    /// values in gameplay code.
    ///
    /// ## Example
    /// ```
    /// let v = Vec2::new(10, 20.5);
    /// ```
    pub fn new(x: impl Floatify, y: impl Floatify) -> Vec2 {
        Vec2 { x: x.floatify(), y: y.floatify() }
    }

    pub fn as_point(&self) -> Point {
        Point(self.x as i32, self.y as i32)
    }
}

/// ## Description
/// Represents an integer-based coordinate in 2D space.
/// 
/// Unlike [Vec2], which is used for continuous floating-point
/// calculations, `Point` is typically used for:
/// - Pixel coordinates
/// - Grid positions
/// - SDL interaction
/// - Discrete spatial checks
/// 
/// - **Item-Type**: Mathematical Primitive
/// 
/// ## Example
/// ```
/// let p = Point(100, 200);
/// if p.in_area(Point(0, 0), Point(1920, 1080)) {
///     println!("Inside screen bounds");
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    /// ## Description
    /// Calculate whether *this* point is contained within a rectangle, composed by the points `p1` (upper left corner) and
    /// `p2` (bottom right corner). Returns `true` if that's the case.
    pub fn in_area(&self, p1: Point, p2: Point) -> bool {
        let min_x = p1.x().min(p2.x());
        let max_x = p1.x().max(p2.x());
        let min_y = p1.y().min(p2.y());
        let max_y = p1.y().max(p2.y());

        self.x() >= min_x && self.x() <= max_x &&
        self.y() >= min_y && self.y() <= max_y
    }

    /// ## Description
    /// Converts this integer-based [`Point`] into a floating-point [`Vec2`].
    ///
    /// Useful when transitioning from discrete coordinate systems
    /// (e.g. mouse input) into physics or rendering calculations.
    ///
    /// ## Example
    /// ```
    /// let p = Point(10, 20);
    /// let v = p.as_vec2();
    /// ```
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x() as f32, self.y() as f32)
    } 
}

/// ## Description
/// Collection of common mathematical utility functions used
/// throughout the engine.
/// 
/// Provides helpers for:
/// - Interpolation
/// - Range mapping
/// - Angle conversion
/// - Floating-point comparison
/// - Vector rotation
/// 
/// Centralizing these operations ensures consistent mathematical
/// behavior across physics, animation, and rendering systems.
/// 
/// - **Item-Type**: Math Utility Namespace
/// 
/// ## Example
/// ```
/// let progress = Math::inverse_lerp(75.0, 0.0, 100.0);
/// let smooth = Math::smoothstep(progress);
/// ```
pub struct Math;

impl Math {
    /// ## Description
    /// Performs **linear interpolation** between two scalar values.
    ///
    /// Linear interpolation computes a value along the straight line between
    /// `v0` and `v1`, controlled by the interpolation factor `t`.
    /// It is commonly used for animations, blending values, time-based movement,
    /// and gradual transitions.
    ///
    /// The mathematical formula is:
    /// `v0 + t * (v1 - v0)`
    ///
    /// - v0: starting value (returned when t = 0.0)
    /// - v1: ending value (returned when t = 1.0)
    /// - t: interpolation factor (not clamped; values outside 0.0..=1.0 extrapolate)
    /// ## Example
    /// ```
    /// let value = Math::lerp(0.0, 10.0, 0.25);
    /// assert_eq!(value, 2.5);
    /// ```
    pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
        v0 + t * (v1 - v0)
    }

    /// ## Description
    /// Linear interpolation between two [`Vec2`]
    pub fn lerp_vec(v0: Vec2, v1: Vec2, t: f32) -> Vec2 {
        v0 + (v1 - v0) * t
    }

    /// ## Description
    /// Restricts a value to lie within a given inclusive range.
    ///
    /// If the value is smaller than `min`, `min` is returned.
    /// If the value is larger than `max`, `max` is returned.
    /// Otherwise, the original value is returned unchanged.
    ///
    /// This is commonly used to enforce bounds on user input,
    /// physics values, colors, or interpolation factors.
    ///
    /// - value: input value to restrict
    /// - min: lower bound
    /// - max: upper bound
    /// ## Example
    /// ```
    /// let v = Math::clamp(1.5, 0.0, 1.0);
    /// assert_eq!(v, 1.0);
    /// ```
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        value.max(min).min(max)
    }

    /// ## Description
    /// Computes the **inverse linear interpolation** of a value within a range.
    ///
    /// This function determines how far `value` lies between `min` and `max`
    /// as a normalized parameter `t` in the range 0.0..=1.0 (if within bounds).
    ///
    /// It is the inverse operation of `lerp` and is frequently used for
    /// normalization, progress calculation, and remapping values.
    ///
    /// The mathematical formula is:
    /// `(value - min) / (max - min)`
    ///
    /// - value: value inside the range
    /// - min: range start
    /// - max: range end
    /// ## Example
    /// ```
    /// let t = Math::inverse_lerp(75.0, 50.0, 100.0);
    /// assert_eq!(t, 0.5);
    /// ```
    pub fn inverse_lerp(value: f32, min: f32, max: f32) -> f32 {
        if min == max {
            0.0
        } else {
            (value - min) / (max - min)
        }
    }

    /// ## Description
    /// Remaps a value from one numerical range into another.
    ///
    /// This function first normalizes the input value using inverse linear
    /// interpolation, then applies linear interpolation to the output range.
    ///
    /// It is useful for converting units, scaling input values, or mapping
    /// sensor/input data to a different domain.
    ///
    /// - value: input value
    /// - in_min: input range start
    /// - in_max: input range end
    /// - out_min: output range start
    /// - out_max: output range end
    /// ## Example
    /// ```
    /// let v = Math::remap(0.5, 0.0, 1.0, 0.0, 100.0);
    /// assert_eq!(v, 50.0);
    /// ```
    pub fn remap(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        let t = Math::inverse_lerp(value, in_min, in_max);
        Math::lerp(out_min, out_max, t)
    }

    /// ## Description
    /// Applies **smoothstep interpolation** to a value.
    ///
    /// Smoothstep produces a smooth, ease-in/ease-out transition between 0 and 1
    /// using a cubic Hermite polynomial. Unlike linear interpolation, the slope
    /// at both ends is zero, eliminating sharp transitions.
    ///
    /// This is commonly used in animations, procedural generation, and shaders.
    ///
    /// - t: interpolation factor (automatically clamped to 0.0..=1.0)
    /// ## Example
    /// ```
    /// let v = Math::smoothstep(0.5);
    /// ```
    pub fn smoothstep(t: f32) -> f32 {
        let t = Math::clamp(t, 0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// ## Description
    /// Returns the mathematical sign of a value.
    ///
    /// This function indicates whether a number is positive, negative, or zero.
    /// It is often used in physics, control logic, and directional calculations.
    ///
    /// - value: input value
    /// ## Example
    /// ```
    /// assert_eq!(Math::sign(-3.0), -1.0);
    /// assert_eq!(Math::sign(0.0), 0.0);
    /// assert_eq!(Math::sign(4.0), 1.0);
    /// ```
    pub fn sign(value: f32) -> f32 {
        if value > 0.0 {
            1.0
        } else if value < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    /// ## Description
    /// Converts an angle from degrees to radians.
    ///
    /// Radians are the standard unit used by most trigonometric functions
    /// in Rust and other programming languages.
    ///
    /// - degrees: angle measured in degrees
    /// ## Example
    /// ```
    /// let r = Math::deg_to_rad(180.0);
    /// assert_eq!(r, std::f32::consts::PI);
    /// ```
    pub fn deg_to_rad(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }

    /// ## Description
    /// Converts an angle from radians to degrees.
    ///
    /// This is useful when presenting angles to users or working with
    /// systems that expect degrees instead of radians.
    ///
    /// - radians: angle measured in radians
    /// ## Example
    /// ```
    /// let d = Math::rad_to_deg(std::f32::consts::PI);
    /// assert_eq!(d, 180.0);
    /// ```
    pub fn rad_to_deg(radians: f32) -> f32 {
        radians * 180.0 / std::f32::consts::PI
    }

    /// ## Description
    /// Checks whether two floating-point numbers are approximately equal.
    ///
    /// Due to floating-point precision errors, exact equality comparisons
    /// are often unreliable. This function considers two values equal if
    /// their absolute difference is within a small tolerance.
    ///
    /// - a: first value
    /// - b: second value
    /// - epsilon: maximum allowed absolute difference
    /// ## Example
    /// ```
    /// let equal = Math::approx_eq(0.1 + 0.2, 0.3, 1e-6);
    /// assert!(equal);
    /// ```
    pub fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() <= epsilon
    }

    /// ## Description
    /// Rotates a 2D vector around the origin by a given angle in degrees.
    ///
    /// The rotation is counterclockwise and uses the standard 2D rotation matrix:
    ///
    /// x' = x * cos(θ) - y * sin(θ)
    /// y' = x * sin(θ) + y * cos(θ)
    ///
    /// - vec: vector to rotate
    /// - degrees: rotation angle in degrees
    ///
    /// ## Example
    /// ```
    /// let v = Vec2::new(1.0, 0.0);
    /// let rotated = Math::rotate_vector(v, 90.0);
    /// assert!(Math::approx_eq(rotated.x, 0.0, 1e-6));
    /// assert!(Math::approx_eq(rotated.y, 1.0, 1e-6));
    /// ```
    pub fn rotate_vector(vec: Vec2, degrees: f32) -> Vec2 {
        let rad = Math::deg_to_rad(degrees);
        let cos = rad.cos();
        let sin = rad.sin();

        Vec2 {
            x: vec.x * cos - vec.y * sin,
            y: vec.x * sin + vec.y * cos,
        }
    }

    /// ## Description
    /// Applies a delta rotation and keeps the result within
    /// the 0°–360° range.
    ///
    /// Uses Euclidean remainder to prevent negative angles
    /// or overflow beyond one full rotation.
    ///
    /// Useful for:
    /// - Sprite rotation
    /// - Camera rotation
    /// - Continuous angular updates
    ///
    /// ## Example
    /// ```
    /// let rotation = Math::rotate(350.0, 20.0);
    /// assert_eq!(rotation, 10.0);
    /// ```
    pub fn rotate(current_rotation: f32, delta_degrees: f32) -> f32 {
        (current_rotation + delta_degrees).rem_euclid(360.0)
    }
}