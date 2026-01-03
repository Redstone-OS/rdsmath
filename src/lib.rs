//! # RDSMath - Minimal Math Library for RedstoneOS
//!
//! Biblioteca matemática minimalista para ambientes `#![no_std]`.
//!
//! ## Características
//!
//! - Zero dependências externas
//! - Funções otimizadas para precisão adequada em gráficos
//! - Todas as funções são `const fn` quando possível
//!
//! ## Módulos
//!
//! | Módulo | Descrição |
//! |--------|-----------|
//! | [`consts`] | Constantes matemáticas (PI, E, etc.) |
//! | [`trig`] | Funções trigonométricas |
//! | [`exp`] | Exponencial, logaritmo, potência |
//! | [`round`] | Arredondamento e truncamento |
//! | [`util`] | Funções utilitárias (min, max, clamp, lerp) |

#![no_std]
#![allow(dead_code)]

pub mod consts;
pub mod exp;
pub mod round;
pub mod trig;
pub mod util;

// =============================================================================
// RE-EXPORTS
// =============================================================================

pub use consts::*;
pub use exp::*;
pub use round::*;
pub use trig::*;
pub use util::*;
