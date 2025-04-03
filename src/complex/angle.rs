use std::f64::consts::PI;

/// Represents an angle measurement that can be expressed in either degrees or radians.
/// 
/// The `Angle` type provides a safe and convenient way to work with angles, ensuring
/// that conversions between degrees and radians are handled correctly. It maintains
/// the internal representation in either degrees or radians as originally specified,
/// performing conversions only when needed.
/// 
/// # Examples
/// 
/// ```
/// use rusticle::Angle;
/// 
/// // Create an angle in degrees
/// let deg_angle = Angle::from_degrees(90.0);
/// 
/// // Create an angle in radians
/// let rad_angle = Angle::from_radians(std::f64::consts::PI / 2.0);
/// 
/// // Both angles represent the same value
/// assert!((deg_angle.to_radians() - rad_angle.to_radians()).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Angle {
    /// Represents an angle measured in degrees (0-360).
    /// 
    /// While any float value is accepted, you can normalize the angle to the
    /// range [0, 360) using the `normalize()` method.
    Degree(f64),

    /// Represents an angle measured in radians (0-2π).
    /// 
    /// While any float value is accepted, you can normalize the angle to the
    /// equivalent of [0, 360) degrees using the `normalize()` method.
    Radian(f64),
}

impl Angle {
    /// Creates a new angle from a value in degrees.
    /// 
    /// This method stores the angle internally as degrees. The value is stored
    /// as-is without normalization. Use `normalize()` if you need the angle
    /// in the range [0, 360).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Angle;
    /// 
    /// let angle = Angle::from_degrees(90.0);
    /// assert_eq!(angle.to_degrees(), 90.0);
    /// 
    /// // Values outside 0-360 are allowed
    /// let large_angle = Angle::from_degrees(720.0); // Two full rotations
    /// assert_eq!(large_angle.normalize().to_degrees(), 0.0);
    /// ```
    pub fn from_degrees(degrees: f64) -> Self {
        Angle::Degree(degrees)
    }

    /// Creates a new angle from a value in radians.
    /// 
    /// This method stores the angle internally as radians. The value is stored
    /// as-is without normalization. Use `normalize()` if you need the angle
    /// in the standard range.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Angle;
    /// use std::f64::consts::PI;
    /// 
    /// let angle = Angle::from_radians(PI / 2.0);
    /// assert_eq!(angle.to_degrees(), 90.0);
    /// 
    /// // Values outside 0-2π are allowed
    /// let large_angle = Angle::from_radians(4.0 * PI); // Two full rotations
    /// assert_eq!(large_angle.normalize().to_degrees(), 0.0);
    /// ```
    pub fn from_radians(radians: f64) -> Self {
        Angle::Radian(radians)
    }

    /// Converts the angle to degrees, regardless of its internal representation.
    /// 
    /// This method performs the conversion from radians to degrees if necessary.
    /// The returned value is not normalized and may be outside the range [0, 360).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Angle;
    /// use std::f64::consts::PI;
    /// 
    /// let deg = Angle::from_degrees(180.0);
    /// assert_eq!(deg.to_degrees(), 180.0);
    /// 
    /// let rad = Angle::from_radians(PI);
    /// assert_eq!(rad.to_degrees(), 180.0);
    /// ```
    pub fn to_degrees(&self) -> f64 {
        match self {
            Angle::Degree(deg) => *deg,
            Angle::Radian(rad) => rad * 180.0 / PI,
        }
    }

    /// Converts the angle to radians, regardless of its internal representation.
    /// 
    /// This method performs the conversion from degrees to radians if necessary.
    /// The returned value is not normalized and may be outside the range [0, 2π).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Angle;
    /// use std::f64::consts::PI;
    /// 
    /// let rad = Angle::from_radians(PI / 2.0);
    /// assert_eq!(rad.to_radians(), PI / 2.0);
    /// 
    /// let deg = Angle::from_degrees(90.0);
    /// assert_eq!(deg.to_radians(), PI / 2.0);
    /// ```
    pub fn to_radians(&self) -> f64 {
        match self {
            Angle::Degree(deg) => deg * PI / 180.0,
            Angle::Radian(rad) => *rad,
        }
    }

    /// Returns a new angle in degrees, converting if necessary.
    /// 
    /// Unlike `to_degrees()` which returns a raw f64 value, this method
    /// returns a new `Angle` instance with the internal representation
    /// stored in degrees.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Angle;
    /// use std::f64::consts::PI;
    /// 
    /// let rad = Angle::from_radians(PI);
    /// let deg = rad.as_degrees();
    /// 
    /// match deg {
    ///     Angle::Degree(d) => assert_eq!(d, 180.0),
    ///     _ => panic!("Should be in degrees"),
    /// }
    /// ```
    pub fn as_degrees(&self) -> Self {
        Angle::Degree(self.to_degrees())
    }

    /// Returns a new angle in radians, converting if necessary.
    /// 
    /// Unlike `to_radians()` which returns a raw f64 value, this method
    /// returns a new `Angle` instance with the internal representation
    /// stored in radians.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Angle;
    /// use std::f64::consts::PI;
    /// 
    /// let deg = Angle::from_degrees(180.0);
    /// let rad = deg.as_radians();
    /// 
    /// match rad {
    ///     Angle::Radian(r) => assert_eq!(r, PI),
    ///     _ => panic!("Should be in radians"),
    /// }
    /// ```
    pub fn as_radians(&self) -> Self {
        Angle::Radian(self.to_radians())
    }

    /// Normalizes the angle to be in the range [0, 360) degrees.
    /// 
    /// This method converts any angle to its equivalent in the range [0, 360) degrees.
    /// It handles both positive and negative angles correctly:
    /// - Positive angles > 360° will be wrapped to their equivalent in [0, 360)
    /// - Negative angles will be converted to their positive equivalent
    /// 
    /// The result is always returned as a degree measurement.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Angle;
    /// 
    /// // Handling angles > 360°
    /// let large = Angle::from_degrees(400.0);
    /// assert_eq!(large.normalize().to_degrees(), 40.0);
    /// 
    /// // Handling negative angles
    /// let negative = Angle::from_degrees(-45.0);
    /// assert_eq!(negative.normalize().to_degrees(), 315.0);
    /// 
    /// // Handling multiple rotations
    /// let multiple = Angle::from_degrees(720.0 + 45.0); // Two rotations plus 45°
    /// assert_eq!(multiple.normalize().to_degrees(), 45.0);
    /// ```
    pub fn normalize(&self) -> Self {
        let degrees = self.to_degrees();
        let normalized = degrees % 360.0;
        let result = if normalized < 0.0 { normalized + 360.0 } else { normalized };
        Angle::Degree(result)
    }
}

/// Implements the conversion from f64 to Angle, interpreting the value as degrees.
/// 
/// This implementation allows for convenient creation of angles from numeric literals,
/// always interpreting them as degree measurements.
/// 
/// # Examples
/// 
/// ```
/// use rusticle::Angle;
/// 
/// let angle: Angle = 90.0.into();
/// assert_eq!(angle.to_degrees(), 90.0);
/// ```
impl From<f64> for Angle {
    fn from(degrees: f64) -> Self {
        Angle::from_degrees(degrees)
    }
}
