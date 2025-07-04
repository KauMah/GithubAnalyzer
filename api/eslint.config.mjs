import tseslint from 'typescript-eslint';
import eslintConfigPrettier from 'eslint-config-prettier/flat';

export default tseslint.config(
  tseslint.configs.recommendedTypeChecked,
  {
    ignores: [
      'node_modules/**',
      'build/**',
      '.pnpm-store',
      'eslint.config.mjs',
    ],
  },
  {
    languageOptions: {
      parserOptions: {
        projectService: true,
      },
    },
    rules: {
      // your rules go here
      // '@typescript-eslint/explicit-function-return-type': 'warn',
      '@typescript-eslint/no-unused-vars': [
        'warn',
        { argsIgnorePattern: '^_' },
      ],
      quotes: ['error', 'single'],
      '@typescript-eslint/array-type': 'off',
      '@typescript-eslint/consistent-type-definitions': 'off',

      '@typescript-eslint/consistent-type-imports': [
        'warn',
        {
          prefer: 'type-imports',
          fixStyle: 'inline-type-imports',
        },
      ],
    },
  },
  eslintConfigPrettier,
);
