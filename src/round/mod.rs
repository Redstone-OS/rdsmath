//! # Funções de Arredondamento
//!
//! Funções para arredondamento, truncamento e outras operações de ponto flutuante.

// =============================================================================
// FLOOR / CEIL / ROUND / TRUNC
// =============================================================================

/// Arredonda para baixo (floor).
///
/// Retorna o maior inteiro menor ou igual ao valor.
#[inline]
pub fn floorf(x: f32) -> f32 {
    let xi = x as i32;
    let xf = xi as f32;
    if x < xf {
        xf - 1.0
    } else {
        xf
    }
}

/// Arredonda para cima (ceil).
///
/// Retorna o menor inteiro maior ou igual ao valor.
#[inline]
pub fn ceilf(x: f32) -> f32 {
    let xi = x as i32;
    let xf = xi as f32;
    if x > xf {
        xf + 1.0
    } else {
        xf
    }
}

/// Arredonda para o inteiro mais próximo.
///
/// Usa arredondamento "half away from zero" (0.5 -> 1, -0.5 -> -1).
#[inline]
pub fn roundf(x: f32) -> f32 {
    if x >= 0.0 {
        floorf(x + 0.5)
    } else {
        ceilf(x - 0.5)
    }
}

/// Trunca para inteiro (remove parte decimal).
#[inline]
pub fn truncf(x: f32) -> f32 {
    x as i32 as f32
}

/// Parte fracionária (x - floor(x)).
#[inline]
pub fn fractf(x: f32) -> f32 {
    x - floorf(x)
}

// =============================================================================
// ABS / SIGN / COPYSIGN
// =============================================================================

/// Valor absoluto.
#[inline]
pub fn absf(x: f32) -> f32 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

/// Sinal do valor (-1.0, 0.0, ou 1.0).
#[inline]
pub fn signf(x: f32) -> f32 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// Copia o sinal de y para x.
#[inline]
pub fn copysignf(x: f32, y: f32) -> f32 {
    let abs_x = absf(x);
    if y >= 0.0 {
        abs_x
    } else {
        -abs_x
    }
}

// =============================================================================
// MOD / REM
// =============================================================================

/// Resto da divisão (x mod y).
#[inline]
pub fn fmodf(x: f32, y: f32) -> f32 {
    x - truncf(x / y) * y
}

/// Resto euclidiano (sempre positivo).
#[inline]
pub fn remf(x: f32, y: f32) -> f32 {
    let r = fmodf(x, y);
    if r < 0.0 {
        r + absf(y)
    } else {
        r
    }
}

// =============================================================================
// F64 VERSIONS
// =============================================================================

/// Floor para f64.
#[inline]
pub fn floor(x: f64) -> f64 {
    let xi = x as i64;
    let xf = xi as f64;
    if x < xf {
        xf - 1.0
    } else {
        xf
    }
}

/// Ceil para f64.
#[inline]
pub fn ceil(x: f64) -> f64 {
    let xi = x as i64;
    let xf = xi as f64;
    if x > xf {
        xf + 1.0
    } else {
        xf
    }
}

/// Round para f64.
#[inline]
pub fn round(x: f64) -> f64 {
    if x >= 0.0 {
        floor(x + 0.5)
    } else {
        ceil(x - 0.5)
    }
}

/// Truncate para f64.
#[inline]
pub fn trunc(x: f64) -> f64 {
    x as i64 as f64
}

/// Abs para f64.
#[inline]
pub fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}
