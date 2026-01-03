//! # Constantes Matemáticas
//!
//! Constantes matemáticas fundamentais.

/// π (pi) - Razão entre circunferência e diâmetro.
pub const PI: f32 = 3.14159265358979323846;

/// τ (tau) - 2π, uma volta completa em radianos.
pub const TAU: f32 = 6.28318530717958647692;

/// π/2 - 90 graus em radianos.
pub const FRAC_PI_2: f32 = 1.57079632679489661923;

/// π/3 - 60 graus em radianos.
pub const FRAC_PI_3: f32 = 1.04719755119659774615;

/// π/4 - 45 graus em radianos.
pub const FRAC_PI_4: f32 = 0.78539816339744830962;

/// π/6 - 30 graus em radianos.
pub const FRAC_PI_6: f32 = 0.52359877559829887308;

/// 1/π
pub const FRAC_1_PI: f32 = 0.31830988618379067154;

/// 2/π
pub const FRAC_2_PI: f32 = 0.63661977236758134308;

/// e (número de Euler)
pub const E: f32 = 2.71828182845904523536;

/// log₂(e)
pub const LOG2_E: f32 = 1.44269504088896340736;

/// log₁₀(e)
pub const LOG10_E: f32 = 0.43429448190325182765;

/// ln(2)
pub const LN_2: f32 = 0.69314718055994530942;

/// ln(10)
pub const LN_10: f32 = 2.30258509299404568402;

/// √2
pub const SQRT_2: f32 = 1.41421356237309504880;

/// 1/√2
pub const FRAC_1_SQRT_2: f32 = 0.70710678118654752440;

/// Graus para radianos (multiplicador).
pub const DEG_TO_RAD: f32 = PI / 180.0;

/// Radianos para graus (multiplicador).
pub const RAD_TO_DEG: f32 = 180.0 / PI;

/// Converte graus para radianos.
#[inline]
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * DEG_TO_RAD
}

/// Converte radianos para graus.
#[inline]
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * RAD_TO_DEG
}
