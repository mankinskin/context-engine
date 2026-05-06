import { fileURLToPath } from 'url';
import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

const viewerApiSrc = fileURLToPath(new URL('../../../../viewer-api/tools/viewer/viewer-api/frontend/ts/src', import.meta.url));
const isStatic = !!process.env.VITE_STATIC_MODE;

export default defineConfig({
  plugins: [preact()],
  define: {
    'import.meta.env.VITE_STATIC_MODE': JSON.stringify(isStatic ? 'true' : ''),
  },
  base: process.env.VITE_BASE_URL || '/',
  build: {
    outDir: isStatic ? 'dist' : '../static',
    emptyOutDir: true,
  },
  resolve: {
    dedupe: ['preact', '@preact/signals', '@preact/signals-core'],
    alias: {
      '@context-engine/viewer-api-frontend': viewerApiSrc,
      'react': 'preact/compat',
      'react-dom': 'preact/compat',
    },
  },
  optimizeDeps: {
    exclude: ['@context-engine/viewer-api-frontend'],
  },
  server: {
    proxy: {
      '/api': 'http://localhost:3000'
    }
  }
});
