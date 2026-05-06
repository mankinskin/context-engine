import { fileURLToPath } from 'url';
import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

const viewerApiSrc = fileURLToPath(new URL('../../../../viewer-api/tools/viewer/viewer-api/frontend/ts/src', import.meta.url));

export default defineConfig({
  plugins: [preact()],
  build: {
    outDir: '../static',
    emptyOutDir: true,
  },
  resolve: {
    // Ensure only one copy of preact is used (prevents hooks issues with shared components)
    dedupe: ['preact', 'preact/hooks', '@preact/signals'],
    alias: {
      '@context-engine/viewer-api-frontend': viewerApiSrc,
    },
  },
  optimizeDeps: {
    exclude: ['@context-engine/viewer-api-frontend'],
  },
  server: {
    proxy: {
      '/api': 'http://localhost:3001'
    }
  }
});
