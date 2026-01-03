# RDSMath v0.1.0

**Minimal Math Library for RedstoneOS**

Biblioteca matemática minimalista para ambientes `#![no_std]`, sem dependências externas.

## Características

- 🚀 **Zero dependências** - Nenhuma crate externa
- 📦 **`#![no_std]`** - Compatível com bare-metal
- ⚡ **Otimizada** - Aproximações rápidas adequadas para gráficos
- 🔧 **Simples** - API limpa e direta

## Módulos

| Módulo | Descrição |
|--------|-----------|
| [`consts`](src/consts/) | Constantes matemáticas (PI, E, etc.) |
| [`trig`](src/trig/) | Funções trigonométricas (sin, cos, tan, atan2) |
| [`exp`](src/exp/) | Exponencial, logaritmo, potência, sqrt |
| [`round`](src/round/) | Arredondamento e truncamento |
| [`util`](src/util/) | Funções utilitárias (lerp, clamp, smoothstep) |

## Uso

```rust
use rdsmath::*;

// Trigonometria
let angle = PI / 4.0;
let (s, c) = sincosf(angle);

// Raiz quadrada (fast inverse sqrt do Quake)
let root = sqrtf(2.0);

// Interpolação
let value = lerpf(0.0, 100.0, 0.5); // = 50.0
let smooth = smoothstepf(0.0, 1.0, 0.5);

// Clamp
let clamped = clampf(150.0, 0.0, 100.0); // = 100.0
```

## Precisão

As funções são otimizadas para velocidade com precisão suficiente para aplicações gráficas (~6 dígitos significativos para a maioria das funções).

| Função | Precisão | Método |
|--------|----------|--------|
| `sinf/cosf` | ~6 dígitos | Aproximação Bhaskara |
| `sqrtf` | ~7 dígitos | Newton-Raphson + IEEE 754 hack |
| `expf/logf` | ~5 dígitos | Decomposição + Taylor |
| `powf` | ~5 dígitos | exp(y * log(x)) |

## Licença

MIT License - RedstoneOS Team
