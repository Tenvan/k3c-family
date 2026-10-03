import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// Frontend von k3c-dev: `npx vite` läuft ohne Wails gegen den Mock (src/api/mock.ts), `wails build` bettet dist/ ein.
export default defineConfig({
  plugins: [react()],
  // fs.allow: der Mock liest internal/planning/page.html, dieselbe Seite, die Go einbettet.
  server: { port: 5181, strictPort: true, fs: { allow: ['..'] } },
});
