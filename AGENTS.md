# Repository Guidelines

## Project Structure & Module Organization
- `src/lib.rs`: Cloudflare Worker entry (`#[event(fetch)]`).
- `src/models.rs`: Request/response types (`GraphRequest`, `GraphType`, etc.).
- `src/charts/`: Chart implementations (`line.rs`, `bar.rs`, `pie.rs`, `area.rs`, `radar.rs`, `scatter.rs`) and factory in `mod.rs` (`create_chart`, `Chart` trait).
- `src/utils/`: Rendering helpers (`svg.rs`, `png.rs`).
- `assets/`: Embedded font(s) used at runtime (e.g., `MPLUS1p-Regular.ttf`).
- `docs/`: Architecture and API guides.  `images/`: Sample outputs.
- `wrangler.toml`: Worker build/run config. `justfile`: Dev commands.

## Build, Test, and Development Commands
- Dev server: `just dev` (or `npx wrangler dev`) → http://localhost:8787
- Build (dev/release auto): `just build`
- Test: `just test` (runs `cargo test`)
- Lint: `just lint` (Clippy, denies warnings)
- Format: `just fmt` / `just fmt-check`
- Deploy: `just deploy`
- Clean: `just clean`
- End‑to‑end curl samples: `just test-all` (writes PNGs to `images/`)

## Coding Style & Naming Conventions
- Rust 2021; format with `rustfmt`; keep Clippy clean (`-D warnings`).
- Files/modules: `snake_case` (e.g., `line.rs`, `svg.rs`). Types/traits: `UpperCamelCase` (e.g., `GraphRequest`, `Chart`). Functions/vars: `snake_case`.
- Prefer small, pure helpers in `utils/`; chart‑specific logic stays in `src/charts/*`.

## Testing Guidelines
- Add unit tests in `#[cfg(test)] mod tests { ... }` next to code.
- Focus on: SVG string generation (deterministic snippets), axis/tick helpers, color selection.
- Run with `just test`. Add regression samples to `images/` when visual behavior changes.

## Commit & Pull Request Guidelines
- Follow Conventional Commits (emoji optional): `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`, `release:` (seen in history).
- PRs must include: summary, linked issue, before/after notes, and sample outputs (`images/*.png`) for rendering changes.
- Pre‑push checklist: `just fmt-check && just lint && just build`.

## Extending Charts (Quick Recipe)
- Add `src/charts/<name>.rs` implementing `Chart` for your type.
- Register it in `src/charts/mod.rs` (`create_chart` match) and the `GraphType` enum in `models.rs` if needed.
- Provide sample curl in README and `just test-all`.

## Security & Config Tips
- Do not commit secrets; use `wrangler secret put` when needed.
- Keep embedded assets small; fonts live under `assets/` and are included via `include_bytes!`.
