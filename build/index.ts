import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { build } from "esbuild";

const dir = dirname(fileURLToPath(import.meta.url));

build({
  entryPoints: [resolve(dir, "modules.ts")],
  entryNames: "index",
  outdir: "dist",
  platform: "neutral",
  format: "esm",
  bundle: true,
  minify: true,
});
