# Rust Embedded Systems Journey

Structured, self-taught path from Rust fundamentals to bare-metal firmware,
built in public — following [Comprehensive Rust](https://google.github.io/comprehensive-rust/)
(Google's Android team course) as the primary reading material, paired with
original exercises and capstone projects per phase.

## 🗺️ Roadmap

| Phase | Focus                             | Status | Capstone Project |
|-------|------------------------------------|--------|-------------------|
| 1     | Fundamentals & CLI                  | ⬜     | [CLI Task Tracker](./phase-1-fundamentals) |
| 2     | Ownership, Lifetimes, Structs       | ⬜     | [In-Memory KV Store](./phase-2-ownership-architecture) |
| 3     | Traits, Generics, Error Handling, Concurrency | ⬜ | [Plugin-Based Data Pipeline](./phase-3-advanced-concepts) |
| 4     | Unsafe, FFI, Bitwise, I/O            | ⬜     | [Software UART / Protocol Decoder](./phase-4-systems-rust) |
| 5     | Embedded Rust (HAL, no_std)          | ⬜     | [Bare-Metal Sensor Node](./phase-5-embedded-rust) |
| 6     | Idiomatic Rust & Unsafe Deep-Dive    | ⬜     | [Embedded-HAL-Style API](./phase-6-idiomatic-and-unsafe) |

⬜ Not started · 🟨 In progress · ✅ Complete

## 🎯 Long-Term Goal
Transitioning into Embedded Systems / Firmware Engineering — bare-metal
programming, RTOS integration, and hardware-software co-design.

## 📖 Primary Resource
[Comprehensive Rust](https://google.github.io/comprehensive-rust/) — Google's
internal Rust training, used as the main reading source. Each phase README
links the relevant chapters. Concepts are practiced in `exercises/`; capstone
projects are original, not from the course.

## 🛠️ Tooling
- `rustup`, stable + `thumbv7em-none-eabihf` target
- `probe-rs` / `cargo-embed` for flashing & debugging
- Hardware: STM32F103C8T6 (Blue Pill) / ESP32 (via `esp-rs`) — see per-phase READMEs

## 📂 Structure
Each phase directory is a self-contained Cargo workspace:
```
phase-N-name/
├── README.md          # concepts, chapter references, embedded rationale
├── exercises/          # small targeted programs, one crate or bin per concept
└── capstone-project/    # the portfolio-grade deliverable for the phase
```
Phase 3 also contains a `concurrency/` subfolder (threads, channels, async
basics) — practiced alongside the phase's core content, not a separate phase.

## 📖 Progress Log
See individual phase READMEs for detailed notes, or [PROGRESS.md](./PROGRESS.md)
for a running dev-log of what I learned and where I got stuck.
