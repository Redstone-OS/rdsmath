//! # Funções Exponenciais e Logarítmicas
//!
//! Exponencial, logaritmo, potência e raiz quadrada.

use crate::consts::LN_2;
use crate::round::absf;

// =============================================================================
// SQRT
// =============================================================================

/// Raiz quadrada.
///
/// Usa o método de Newton-Raphson para convergência rápida.
#[inline]
pub fn sqrtf(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }

    // Estimativa inicial usando bit manipulation
    // IEEE 754 hack (similar ao famoso inverse sqrt do Quake)
    let mut y = x;
    let mut i = y.to_bits();
    i = 0x5f3759df - (i >> 1); // Magic number
    y = f32::from_bits(i);
    y = 1.0 / y; // Inverter para obter sqrt em vez de 1/sqrt

    // 2 iterações de Newton-Raphson para precisão
    y = 0.5 * (y + x / y);
    y = 0.5 * (y + x / y);

    y
}

/// Raiz quadrada inversa (1/sqrt).
///
/// Muito rápido, útil para normalização de vetores.
#[inline]
pub fn rsqrtf(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }

    // Fast inverse square root (Quake III)
    let mut y = x;
    let mut i = y.to_bits();
    i = 0x5f3759df - (i >> 1);
    y = f32::from_bits(i);

    // 1 iteração de Newton-Raphson
    y = y * (1.5 - 0.5 * x * y * y);

    y
}

/// Raiz cúbica.
#[inline]
pub fn cbrtf(x: f32) -> f32 {
    if x == 0.0 {
        return 0.0;
    }

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = absf(x);

    // Estimativa inicial
    let mut y = x;
    let mut i = y.to_bits();
    i = i / 3 + 0x2a508f8a; // Magic number para cbrt
    y = f32::from_bits(i);

    // Newton-Raphson para cbrt: y = (2y + x/y²) / 3
    y = (2.0 * y + x / (y * y)) / 3.0;
    y = (2.0 * y + x / (y * y)) / 3.0;

    sign * y
}

// =============================================================================
// EXP / LOG
// =============================================================================

/// Exponencial (e^x).
#[inline]
pub fn expf(x: f32) -> f32 {
    // Limites para evitar overflow/underflow
    if x > 88.0 {
        return f32::MAX;
    }
    if x < -88.0 {
        return 0.0;
    }

    // Usa identidade: e^x = 2^(x/ln2)
    // e^x = 2^k * e^r onde k = floor(x/ln2) e r = x - k*ln2

    let k = (x / LN_2) as i32;
    let r = x - (k as f32) * LN_2;

    // Aproximação polinomial de e^r para r pequeno
    // e^r ≈ 1 + r + r²/2 + r³/6 + r⁴/24 + r⁵/120
    let r2 = r * r;
    let r3 = r2 * r;
    let r4 = r2 * r2;
    let r5 = r4 * r;

    let exp_r = 1.0 + r + r2 / 2.0 + r3 / 6.0 + r4 / 24.0 + r5 / 120.0;

    // Multiplica por 2^k
    exp_r * exp2f(k as f32)
}

/// 2^x (mais eficiente que expf para bases 2).
#[inline]
pub fn exp2f(x: f32) -> f32 {
    if x > 127.0 {
        return f32::MAX;
    }
    if x < -127.0 {
        return 0.0;
    }

    let k = x as i32;
    let f = x - k as f32;

    // 2^k via bit manipulation
    let pow2_k = if k >= 0 {
        (1u32 << k) as f32
    } else {
        1.0 / ((1u32 << (-k)) as f32)
    };

    // Aproximação polinomial de 2^f para f em [0, 1)
    let pow2_f = 1.0 + f * (0.6931472 + f * (0.2402265 + f * (0.0555041 + f * 0.0096139)));

    pow2_k * pow2_f
}

/// Logaritmo natural (ln).
#[inline]
pub fn logf(x: f32) -> f32 {
    if x <= 0.0 {
        return f32::MIN;
    }

    // Decompõe x = m * 2^e onde 1 <= m < 2
    let bits = x.to_bits();
    let e = ((bits >> 23) & 0xff) as i32 - 127;
    let m = f32::from_bits((bits & 0x007fffff) | 0x3f800000);

    // ln(x) = ln(m * 2^e) = ln(m) + e * ln(2)
    // Aproximação polinomial de ln(m) para m em [1, 2)
    let m_minus_1 = m - 1.0;
    let ln_m = m_minus_1 * (1.0 - m_minus_1 * (0.5 - m_minus_1 * (0.333333 - m_minus_1 * 0.25)));

    ln_m + (e as f32) * LN_2
}

/// Logaritmo base 2.
#[inline]
pub fn log2f(x: f32) -> f32 {
    logf(x) / LN_2
}

/// Logaritmo base 10.
#[inline]
pub fn log10f(x: f32) -> f32 {
    logf(x) / 2.302585
}

// =============================================================================
// POW
// =============================================================================

/// Potência (x^y).
#[inline]
pub fn powf(x: f32, y: f32) -> f32 {
    if x == 0.0 {
        return if y > 0.0 { 0.0 } else { f32::MAX };
    }

    if y == 0.0 {
        return 1.0;
    }

    if y == 1.0 {
        return x;
    }

    if y == 2.0 {
        return x * x;
    }

    if y == 0.5 {
        return sqrtf(x);
    }

    // Para inteiros, use multiplicação
    let y_int = y as i32;
    if absf(y - y_int as f32) < 0.0001 {
        return powi(x, y_int);
    }

    // Caso geral: x^y = e^(y * ln(x))
    if x > 0.0 {
        expf(y * logf(x))
    } else {
        // x negativo com expoente não-inteiro
        0.0
    }
}

/// Potência com expoente inteiro.
#[inline]
pub fn powi(x: f32, n: i32) -> f32 {
    if n == 0 {
        return 1.0;
    }

    let mut result = 1.0;
    let mut base = x;
    let mut exp = if n < 0 { -n } else { n } as u32;

    // Exponenciação binária
    while exp > 0 {
        if exp & 1 == 1 {
            result *= base;
        }
        base *= base;
        exp >>= 1;
    }

    if n < 0 {
        1.0 / result
    } else {
        result
    }
}

// =============================================================================
// HYPOT
// =============================================================================

/// Hipotenusa sqrt(x² + y²) sem overflow.
#[inline]
pub fn hypotf(x: f32, y: f32) -> f32 {
    let x = absf(x);
    let y = absf(y);

    let (max, min) = if x > y { (x, y) } else { (y, x) };

    if max == 0.0 {
        return 0.0;
    }

    let ratio = min / max;
    max * sqrtf(1.0 + ratio * ratio)
}
