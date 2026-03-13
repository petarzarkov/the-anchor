# Tauri Vite Template

[![CI](https://github.com/petarzarkov/tauri-vite-template/actions/workflows/ci.yml/badge.svg)](https://github.com/petarzarkov/tauri-vite-template/actions/workflows/ci.yml)

A minimal, production-ready template for building desktop applications with [Tauri 2](https://tauri.app), [Vite](https://vite.dev), and [React 19](https://react.dev).

## Features

- **Tauri 2** — native desktop shell with Rust backend
- **React 19 + Vite 8** — fast frontend with HMR
- **Chakra UI v3** — dark gaming theme with custom button recipes (`solid`, `ghost`, `outline`, `fire`, `glass`)
- **IPC demo** — counter backed by Rust shared state, showcasing `invoke()` with `increment`, `decrement`, `reset`, and `random_increment` commands
- **Bun** — package manager and script runner
- **Biome** — single-tool lint + format (replaces ESLint + Prettier)
- **Husky + lint-staged** — pre-commit hooks
- **GitHub Actions** — parallel CI (frontend + Rust) and multi-platform release (Windows, Linux, macOS)

## Prerequisites

| Tool | Version |
|------|---------|
| [Rust](https://rustup.rs) | stable |
| [Bun](https://bun.sh) | latest |

**Linux** also requires system libraries for Tauri:

```bash
sudo apt-get install -y \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  xdg-utils
```

See the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/) for Windows and macOS requirements.

## Getting Started

```bash
# Install dependencies
bun install

# Start the Tauri dev app (Rust + frontend)
bun run tauri:dev

# Start only the frontend dev server
bun run dev
```

## Scripts

| Script | Description |
|--------|-------------|
| `bun run dev` | Vite frontend dev server (port 1420) |
| `bun run build` | TypeScript check + Vite production build |
| `bun run typecheck` | TypeScript type check only |
| `bun run check` | Biome CI check (no writes — used in CI) |
| `bun run lint` | Biome lint + format with auto-fix |
| `bun run format` | Biome format only |
| `bun run tauri:dev` | Full Tauri dev mode |
| `bun run tauri:build` | Build Tauri release bundle |

## Project Structure

```
├── app/                   # React frontend
│   ├── App.tsx            # Root component (counter demo)
│   ├── main.tsx           # Entry point
│   └── theme/
│       ├── system.ts      # Chakra UI system (colors, fonts, keyframes)
│       └── recipes.ts     # Button and dialog recipes
│
├── service/               # Rust / Tauri backend
│   ├── src/main.rs        # App state, Tauri commands, unit tests
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json    # Tauri configuration
│   └── icons/
│
├── .github/
│   ├── actions/setup/     # Reusable setup action (Bun + optional Rust)
│   └── workflows/
│       ├── ci.yml         # Runs on every push
│       └── release.yml    # Runs on version tags (v*)
│
├── index.html
├── vite.config.ts
├── biome.json
└── package.json
```

## Tauri IPC Commands

The counter demo registers these commands in `service/src/main.rs`:

| Command | Description |
|---------|-------------|
| `increment` | Increments count by 1, returns new value |
| `decrement` | Decrements count by 1, returns new value |
| `reset` | Resets count to 0, returns 0 |
| `random_increment` | Increments by a random 1–10, returns new value |

Call them from the frontend:

```typescript
import { invoke } from '@tauri-apps/api/core';

const count = await invoke<number>('increment');
```

Add new commands in `main.rs` and register them in `invoke_handler!`:

```rust
#[tauri::command]
fn my_command(state: State<'_, AppState>) -> String {
    "hello".to_string()
}

// in main():
.invoke_handler(tauri::generate_handler![
    increment, decrement, reset, random_increment,
    my_command,
])
```

## CI / CD

### CI (`ci.yml`)

Triggered on every push and pull request. Runs two parallel jobs:

- **Frontend** — `biome ci` + `tsc --noEmit` in parallel, then `vite build`. Rust toolchain is skipped.
- **Rust Tests** — installs system deps, runs `cargo test`.

### Release (`release.yml`)

Triggered on tags matching `v*` (e.g. `v1.0.0`). Builds native installers for:

| Platform | Target |
|----------|--------|
| Windows | `x86_64-pc-windows-msvc` |
| Linux | `x86_64-unknown-linux-gnu` |
| macOS | `aarch64-apple-darwin` |

Creates a **draft** GitHub release with the built artifacts.

```bash
git tag v1.0.0
git push origin v1.0.0
```

## Customization

### Replace the counter with your app

1. Add Rust state and commands in `service/src/main.rs`
2. Register them in `invoke_handler!` and `.manage()`
3. Call them from React with `invoke()` from `@tauri-apps/api/core`

### Theme

Edit `app/theme/system.ts` to change colors, fonts, and keyframes. Edit `app/theme/recipes.ts` to modify or add component variants.

> **Note on custom Chakra variants:** variants `fire` and `glass` are defined in the recipe but are not in Chakra's default TypeScript types. Use `variant={"fire" as "solid"}` or run `bunx @chakra-ui/cli typegen` to generate proper types.

## License

MIT
