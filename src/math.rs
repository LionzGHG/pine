
/// # Description
/// Mathematical construct to represent a point in 2d-space. 
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
}

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
}