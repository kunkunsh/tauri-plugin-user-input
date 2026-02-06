import { readFileSync } from 'fs'
import { join } from 'path'
import { cwd } from 'process'
import typescript from '@rollup/plugin-typescript'

const pkg = JSON.parse(readFileSync(join(cwd(), 'package.json'), 'utf8'))

export default {
  input: 'guest-js/index.ts',
  output: [
    {
      dir: 'dist-js',
      entryFileNames: 'index.js',
      format: 'esm'
    },
    {
      dir: 'dist-js',
      entryFileNames: 'index.cjs',
      format: 'cjs'
    }
  ],
  plugins: [
    typescript({
      declaration: true,
      declarationDir: 'dist-js',
      rootDir: 'guest-js'
    })
  ],
  external: [
    /^@tauri-apps\/api/,
    ...Object.keys(pkg.dependencies || {}),
    ...Object.keys(pkg.peerDependencies || {})
  ]
}
