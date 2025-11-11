import js from "@eslint/js";
import reactPlugin from "eslint-plugin-react";
import { FlatCompat } from "@eslint/eslintrc";

const compat = new FlatCompat();

export default [
  js.configs.recommended,
  ...compat.extends("plugin:react/recommended"),
  {
    files: ["**/*.{ts,tsx}"]
  },
  {
    settings: {
      react: {
        version: "detect"
      }
    }
  }
];
