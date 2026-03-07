import js from '@eslint/js';
import tsParser from '@typescript-eslint/parser';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import prettier from 'eslint-plugin-prettier';
import globals from 'globals';
import tseslint from 'typescript-eslint';

import importPlugin from 'eslint-plugin-import';
import unusedImports from 'eslint-plugin-unused-imports';

export default [
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    ignores: ['node_modules/**', 'dist/**', 'build/**', 'target/**', 'src-tauri/target/**'],
  },

  {
    files: ['src/**/*.{ts,tsx}'],
    ignores: ['.editorconfig', '.gitignore', '.prettierrc.js', 'README.md'],

    languageOptions: {
      parser: tsParser,
      parserOptions: {
        project: './tsconfig.json',
        tsconfigRootDir: import.meta.dirname,
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    settings: {
      react: {
        version: 'detect',
      },
    },

    plugins: {
      react,
      'react-hooks': reactHooks,
      prettier,
      import: importPlugin,
      'unused-imports': unusedImports,
    },

    rules: {
      /* ---------------- prettier ---------------- */
      'prettier/prettier': 'error',

      /* ---------------- base ---------------- */
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': 'off',

      /* ---------------- unused imports ---------------- */
      'unused-imports/no-unused-imports': 'error',
      'unused-imports/no-unused-vars': [
        'warn',
        {
          vars: 'all',
          varsIgnorePattern: '^_',
          args: 'after-used',
          argsIgnorePattern: '^_',
        },
      ],

      /* ---------------- import ---------------- */
      'import/order': [
        'warn',
        {
          groups: ['builtin', 'external', 'internal', 'parent', 'sibling', 'index'],
          'newlines-between': 'always',
          alphabetize: { order: 'asc', caseInsensitive: true },
        },
      ],

      /* ---------------- React ---------------- */
      'react/react-in-jsx-scope': 'off',
      'react/jsx-boolean-value': ['warn', 'never'],
      'react/self-closing-comp': 'warn',

      /* ---------------- React Hooks ---------------- */
      'react-hooks/rules-of-hooks': 'error',

      /* ---------------- TypeScript ---------------- */
      '@typescript-eslint/no-floating-promises': [
        'warn',
        {
          ignoreVoid: true,
          ignoreIIFE: true,
        },
      ],
    },
  },
];
