import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

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
  server: {
    proxy: {
      '/api': 'http://localhost:3000'
    }
  }
});
