import { defineConfig } from 'vite';

// Bündelt den Online-Server (src/online/wsServer.ts, ohne Phaser) für Node: dist-server/online.mjs, geladen von server/server.mjs.
export default defineConfig({
  build: {
    ssr: 'src/online/wsServer.ts',
    outDir: 'dist-server',
    emptyOutDir: true,
    target: 'node22',
    rollupOptions: { output: { entryFileNames: 'online.mjs' } },
  },
});
