import antfu from '@antfu/eslint-config'

export default antfu({
  typescript: true,
  test: { overrides: { 'test/no-import-node-test': 'off' } },
  ignores: [
    'public/**',
    'src-tauri/**',
    '**/target/**',
  ],
})
