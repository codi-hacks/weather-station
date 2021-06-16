module.exports = {
  root: true,
  env: {
    node: true
  },
  extends: [
    'plugin:vue/essential',
    '@vue/standard'
  ],
  globals: {
    API_URL: true
  },
  parserOptions: {
    parser: 'babel-eslint'
  },
  rules: {
    'comma-dangle': 'warn',
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-unused-vars': 'warn',
    'space-before-function-paren': ['error', 'never'],
    'spaced-comment': 'off',
    'vue/require-v-for-key': 'off'
  }
}
