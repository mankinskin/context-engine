# .presentation/

Slidev-based presentational entry point for this repository.

- `deck.toml` — deck metadata (`id`, `title`, `summary`, `theme`, `standalone`,
  and — for a composing deck — `composes = [...]` listing the sub-repo deck
  ids it imports).
- `slides.md` — the Slidev deck itself.
- `package.json` — a standalone, independently buildable Slidev toolchain
  (`npm run dev`, `npm run build`).

## Composing a sub-repository's deck

A super-repository imports a sub-repository's slides inline via Slidev's
per-slide `src:` frontmatter, so the combined build renders those slides
without iframing:

```md
---
src: ../workflow-tools/.presentation/slides.md
---
```

The sub-repository's deck still builds standalone — `composes` in `deck.toml`
is metadata only; it does not affect either build.

## Replicating this scaffold in a new repository

1. Copy `package.json` (adjust the `name` field).
2. Copy `deck.toml` and `slides.md`, write your own content.
3. `npm install && npm run dev` for hot reload, `npm run build` for a static
   SPA in `dist/`.

WASM-in-slide embedding (a future ticket, `e01dd058`) will add
`vite-plugin-wasm` + `vite-plugin-top-level-await` back once a WASM component
actually ships in a slide; the current toolchain omits them because the
`vite-plugin-top-level-await` release compatible with this Slidev/Rollup
version fails at build time when unused.
