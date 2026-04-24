import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html', 'cobertura'],
      reportsDirectory: './coverage',
      exclude: [
        'node_modules/',
        'src/test/',
        '**/*.d.ts',
        '**/*.config.*',
        '**/types/**',
      ],
    },
    reporters: ['default', 'junit'],
    outputFile: {
      junit: './report.xml',
    },
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
    // Force a single copy of React across the workspace. Without this,
    // vitest's forked pool workers walk pnpm's virtual store from each
    // peer-dep's own `node_modules/`, picking up multiple React instances
    // and tripping the "Invalid hook call" rule.
    dedupe: ['react', 'react-dom'],
  },
})
