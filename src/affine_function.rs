pub mod affine {
    pub struct AffineCoefficient {
        pub angular: f32,
        pub linear: f32,
    }

    impl AffineCoefficient {
        pub fn is_crescent(coefficients: &AffineCoefficient) -> bool {
            if coefficients.angular > 0.0 {
                return true;
            }
            return false;
        }

        pub fn is_decrescent(coefficients: &AffineCoefficient) -> bool {
            if coefficients.angular < 0.0 {
                return true;
            }
            return false;
        }

        pub fn is_crescent_decrescent_constant(coefficients: &AffineCoefficient) {}

        pub fn zero_of_function(coefficients: &AffineCoefficient) -> f32 {
            (-coefficients.linear) / coefficients.angular
        }

        pub fn function_summary(coefficients: &AffineCoefficient) {
            println!("The function f(x) = {}x + {} and intersects the x axis on ({}, 0) and the y axis on (0, {})", coefficients.angular, coefficients.linear, Self::zero_of_function(&coefficients), coefficients.linear);
        }
    }
}
