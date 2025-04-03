use std::f64::consts::PI;
use rusticle::Angle;

/// Test suite for the Angle type.
mod angle_tests {
    use super::*;

    /// Tests basic conversion between degrees and radians.
    #[test]
    fn test_angle_conversion() {
        let deg = Angle::from_degrees(90.0);
        assert_eq!(deg.to_radians(), PI / 2.0);
        
        let rad = Angle::from_radians(PI);
        assert_eq!(rad.to_degrees(), 180.0);
    }

    /// Tests angle normalization to [0, 360) degree range.
    #[test]
    fn test_angle_normalization() {
        let angle = Angle::from_degrees(400.0);
        assert_eq!(angle.normalize().to_degrees(), 40.0);
        
        let angle = Angle::from_degrees(-45.0);
        assert_eq!(angle.normalize().to_degrees(), 315.0);
    }

    /// Tests roundtrip conversion between degrees and radians.
    #[test]
    fn test_angle_conversions_roundtrip() {
        let original_deg = 45.0;
        let angle = Angle::from_degrees(original_deg);
        let rad = angle.to_radians();
        let back_to_deg = Angle::from_radians(rad).to_degrees();
        assert!((original_deg - back_to_deg).abs() < 1e-10);
    }

    /// Tests the as_degrees and as_radians conversion methods.
    #[test]
    fn test_angle_as_units() {
        let deg = Angle::from_degrees(90.0);
        let in_rad = deg.as_radians();
        assert_eq!(in_rad.to_radians(), PI / 2.0);

        let rad = Angle::from_radians(PI);
        let in_deg = rad.as_degrees();
        assert_eq!(in_deg.to_degrees(), 180.0);
    }
} 