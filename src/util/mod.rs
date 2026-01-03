//! # Funções Utilitárias
//!
//! Funções auxiliares comuns.

// =============================================================================
// MIN / MAX / CLAMP
// =============================================================================

/// Mínimo entre dois valores.
#[inline]
pub const fn minf(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// Máximo entre dois valores.
#[inline]
pub const fn maxf(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Clamp de valor entre min e max.
#[inline]
pub const fn clampf(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Clamp de valor entre 0 e 1 (saturate).
#[inline]
pub const fn saturatef(x: f32) -> f32 {
    clampf(x, 0.0, 1.0)
}

// =============================================================================
// LERP / SMOOTHSTEP
// =============================================================================

/// Interpolação linear entre a e b.
///
/// t=0 retorna a, t=1 retorna b.
#[inline]
pub const fn lerpf(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Interpolação linear inversa.
///
/// Retorna t tal que lerp(a, b, t) = x.
#[inline]
pub fn inv_lerpf(a: f32, b: f32, x: f32) -> f32 {
    if a == b {
        0.0
    } else {
        (x - a) / (b - a)
    }
}

/// Remapeia valor de um range para outro.
#[inline]
pub fn remapf(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let t = inv_lerpf(in_min, in_max, x);
    lerpf(out_min, out_max, t)
}

/// Smoothstep hermite interpolation.
///
/// Retorna 0 se x < edge0, 1 se x > edge1, e interpolação suave entre.
#[inline]
pub fn smoothstepf(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clampf((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smootherstep (Ken Perlin's improved version).
#[inline]
pub fn smootherstepf(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clampf((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

// =============================================================================
// STEP / MIX
// =============================================================================

/// Step function.
///
/// Retorna 0 se x < edge, 1 caso contrário.
#[inline]
pub const fn stepf(edge: f32, x: f32) -> f32 {
    if x < edge {
        0.0
    } else {
        1.0
    }
}

/// Mix (alias para lerp).
#[inline]
pub const fn mixf(a: f32, b: f32, t: f32) -> f32 {
    lerpf(a, b, t)
}

// =============================================================================
// INTEGER VERSIONS
// =============================================================================

/// Mínimo entre dois inteiros.
#[inline]
pub const fn mini(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

/// Máximo entre dois inteiros.
#[inline]
pub const fn maxi(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Clamp de inteiro.
#[inline]
pub const fn clampi(x: i32, min: i32, max: i32) -> i32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Mínimo entre dois u32.
#[inline]
pub const fn minu(a: u32, b: u32) -> u32 {
    if a < b {
        a
    } else {
        b
    }
}

/// Máximo entre dois u32.
#[inline]
pub const fn maxu(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Clamp de u32.
#[inline]
pub const fn clampu(x: u32, min: u32, max: u32) -> u32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

// =============================================================================
// COMPARISON
// =============================================================================

/// Compara floats com tolerância.
#[inline]
pub fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    let diff = a - b;
    (if diff < 0.0 { -diff } else { diff }) < epsilon
}

/// Compara floats com tolerância padrão (1e-6).
#[inline]
pub fn nearly_eq(a: f32, b: f32) -> bool {
    approx_eq(a, b, 1e-6)
}

/// Verifica se é aproximadamente zero.
#[inline]
pub fn is_zero(x: f32, epsilon: f32) -> bool {
    (if x < 0.0 { -x } else { x }) < epsilon
}
