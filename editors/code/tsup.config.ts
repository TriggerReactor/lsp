import { relative, resolve } from 'node:path'
import { defineConfig } from 'tsup'

export default defineConfig({
  clean: true,
  dts: false,
  entry: ['src/index.ts'],
  format: ['cjs'],
  minify: true,
  skipNodeModulesBundle: true,
  sourcemap: true,
  target: 'es2021',
  tsconfig: relative(__dirname, resolve(process.cwd(), 'src', 'tsconfig.json'))
})
