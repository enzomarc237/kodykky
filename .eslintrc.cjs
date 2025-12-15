module.exports = {
  root: true,
  env: { browser: true, es2020: true },
  extends: [
    'eslint:recommended',
  ],
  ignorePatterns: ['dist', '.eslintrc.cjs', 'build/**', 'src-tauri/**'],
  parserOptions: { ecmaVersion: 'latest', sourceType: 'module' },
  rules: {
    'no-unused-vars': 'warn',
    'no-undef': 'off',
    'no-console': 'warn',
  },
}