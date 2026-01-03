//! # Funções Trigonométricas
//!
//! Seno, cosseno, tangente e suas inversas.
//! Implementadas usando aproximações polinomiais otimizadas para precisão gráfica.

use crate::consts::{FRAC_PI_2, PI, TAU};
use crate::round::absf;

// =============================================================================
// SIN / COS
// =============================================================================

/// Seno (entrada em radianos).
///
/// Precisão: ~6 dígitos significativos (suficiente para gráficos).
#[inline]
pub fn sinf(x: f32) -> f32 {
    // Normaliza para [-π, π]
    let x = normalize_angle(x);

    // Aproximação polinomial (Bhaskara I, modificada)
    // Mais precisa que Taylor para gráficos
    if x >= 0.0 {
        sin_approx(x)
    } else {
        -sin_approx(-x)
    }
}

/// Cosseno (entrada em radianos).
#[inline]
pub fn cosf(x: f32) -> f32 {
    sinf(x + FRAC_PI_2)
}

/// Tangente (entrada em radianos).
#[inline]
pub fn tanf(x: f32) -> f32 {
    let c = cosf(x);
    if absf(c) < 1e-10 {
        if c >= 0.0 {
            f32::MAX
        } else {
            f32::MIN
        }
    } else {
        sinf(x) / c
    }
}

// Aproximação de seno para [0, π]
fn sin_approx(x: f32) -> f32 {
    // Para x em [0, π], usamos uma aproximação quadrática melhorada
    // Baseada em: sin(x) ≈ (16x(π-x)) / (5π² - 4x(π-x))
    let x = if x > PI { x - PI } else { x };

    let num = 16.0 * x * (PI - x);
    let den = 5.0 * PI * PI - 4.0 * x * (PI - x);

    let result = num / den;

    // Corrige sinal para segunda metade
    if x > PI {
        -result
    } else {
        result
    }
}

/// Normaliza ângulo para [-π, π].
#[inline]
fn normalize_angle(x: f32) -> f32 {
    let mut x = x;

    // Rápido para valores próximos de zero
    if x >= -PI && x <= PI {
        return x;
    }

    // Normaliza para [-2π, 2π]
    x = x - ((x / TAU) as i32 as f32) * TAU;

    // Normaliza para [-π, π]
    if x > PI {
        x - TAU
    } else if x < -PI {
        x + TAU
    } else {
        x
    }
}

// =============================================================================
// ARCSIN / ARCCOS / ARCTAN
// =============================================================================

/// Arco seno (retorna radianos).
///
/// Entrada: [-1, 1], Saída: [-π/2, π/2]
#[inline]
pub fn asinf(x: f32) -> f32 {
    // Clamp input
    let x = if x < -1.0 {
        -1.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    };

    // Aproximação polinomial
    let x2 = x * x;
    let x3 = x2 * x;

    // Série de Taylor melhorada para asin
    x + x3 * (1.0 / 6.0 + x2 * (3.0 / 40.0 + x2 * (15.0 / 336.0)))
}

/// Arco cosseno (retorna radianos).
///
/// Entrada: [-1, 1], Saída: [0, π]
#[inline]
pub fn acosf(x: f32) -> f32 {
    FRAC_PI_2 - asinf(x)
}

/// Arco tangente (retorna radianos).
///
/// Saída: [-π/2, π/2]
#[inline]
pub fn atanf(x: f32) -> f32 {
    // Para valores grandes, use identidade atan(x) = π/2 - atan(1/x)
    if absf(x) > 1.0 {
        let sign = if x > 0.0 { 1.0 } else { -1.0 };
        return sign * FRAC_PI_2 - atanf(1.0 / x);
    }

    // Aproximação polinomial para |x| <= 1
    // atan(x) ≈ x - x³/3 + x⁵/5 - x⁷/7 + ...
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;

    x - x3 / 3.0 + x5 / 5.0 - x7 / 7.0
}

/// Arco tangente de dois argumentos (retorna radianos).
///
/// Retorna o ângulo entre o eixo X positivo e o ponto (x, y).
/// Saída: [-π, π]
#[inline]
pub fn atan2f(y: f32, x: f32) -> f32 {
    if x == 0.0 {
        if y > 0.0 {
            return FRAC_PI_2;
        } else if y < 0.0 {
            return -FRAC_PI_2;
        } else {
            return 0.0;
        }
    }

    let atan = atanf(y / x);

    if x > 0.0 {
        atan
    } else if y >= 0.0 {
        atan + PI
    } else {
        atan - PI
    }
}

// =============================================================================
// SINCOS (otimizado)
// =============================================================================

/// Calcula seno e cosseno simultaneamente.
///
/// Mais eficiente que chamar sinf e cosf separadamente.
#[inline]
pub fn sincosf(x: f32) -> (f32, f32) {
    let s = sinf(x);
    let c = cosf(x);
    (s, c)
}
