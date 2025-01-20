/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    '../crates/frontend/src/styles.rs'
  ],
  theme: {
    extend: {
      colors: {
        'gray': '#e9e9e9',
        'offblack': '#2c2c2c',
        'lightgray': '#f3f3f3',
        'coreblue': '#007acc',
        'darkgray': '#828282',
        'innergray': '#dfdfdf'
      }
    },
  },
  plugins: [],
}

