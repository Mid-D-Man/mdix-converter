# mdix-converter

Universal data optimizer. Paste JSON, TOML, or YAML — get optimised DixScript `.mdix` back, with automatic `@ENUMS` and `@QUICKFUNCS` induction.

## Live playground

https://mdix-converter.pages.dev

## How it works

1. **Ingest** — Parse input into a format-agnostic IR
2. **Induction** — Frequency-analyse strings for enum candidates; hash object skeletons for function candidates
3. **Name** — Heuristic plural→singular for function names; PascalCase for enums
4. **Emit** — Write idiomatic `.mdix` with aligned `@ENUMS`, `@QUICKFUNCS`, and `@DATA`

## Architecture

```
crates/
  converter-core/   Pure Rust lib — compiles to native + wasm32
  converter-wasm/   wasm-bindgen glue — exposes core to the browser
web/                SvelteKit site — landing page + live playground
```

## License

MIT
