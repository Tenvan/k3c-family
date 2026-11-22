import { writeFileSync } from 'node:fs';
import react from '@vitejs/plugin-react';
import { defineConfig, type Plugin } from 'vite';
import { ALIAS, DEDUPE, MOCK_DATA, WEBKIT } from './paths.ts';
import { waitForBackend } from '../../webkit/lib/devProxy.ts';

const API = process.env.K3C_STUDIO_API ?? 'http://127.0.0.1:8193';

/** Der Build leert dist/; der eingecheckte Platzhalter dist/.gitkeep (go:embed ohne Frontend, main.go) bleibt. */
function keepGitkeep(): Plugin {
  return { name: 'k3c-keep-gitkeep', apply: 'build', closeBundle: () => writeFileSync(new URL('./dist/.gitkeep', import.meta.url), '') };
}

// Frontend von k3c-studio (WZ63.3): `task k3c-studio:dev` startet Vite gegen `k3c-studio -dev` (Proxy /api auf die
// Oberflächen-API, K3C_STUDIO_API, Vorgabe 127.0.0.1:8193), `?mock=1` zeigt den Mock (src/api/mock.ts), `wails build`
// bettet dist/ ein. Ports: Vite 5193 (K3C_STUDIO_UI_PORT), nie 5191 (Workbench) oder 5192/8091/8182 (altes Asset-Studio).
// changeOrigin bleibt aus: Host und Origin (localhost:5193) bestehen die Prüfungen von tools/workkit/uikit. cors aus:
// keine fremde Seite darf Antworten des Dev-Servers lesen.
export default defineConfig({
  plugins: [react(), waitForBackend(API), keepGitkeep()], // /api wartet, bis das Backend bereit ist (auch beim Neustart)
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  server: {
    port: Number(process.env.K3C_STUDIO_UI_PORT ?? 5193),
    strictPort: true,
    cors: false,
    fs: { allow: ['.', WEBKIT, ...MOCK_DATA] },
    proxy: { '/api': { target: API, changeOrigin: false } },
  },
});
