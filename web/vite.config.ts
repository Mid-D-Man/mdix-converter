import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  // Allow Vite to serve WASM files from src/lib/wasm during dev
  server: { fs: { allow: ['..'] } },
  // Ensure .wasm files are served with the correct MIME type
  assetsInclude: ['**/*.wasm'],
});
