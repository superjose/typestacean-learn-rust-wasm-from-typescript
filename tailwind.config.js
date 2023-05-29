/** @type {import('tailwindcss').Config} */
export default {
  mode: "jit",
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "./src/**/*.rs",
    "./src/**/*.css",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
