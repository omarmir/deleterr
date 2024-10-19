import {black, white, green, purple, orange, teal, red} from 'tailwindcss/colors'

export default {
  darkMode: 'class',
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        lato: 'lato, open-sans, sans-serif',
        inter: 'inter, ui-sans-serif, system-ui, sans-serif',
      },
      maxHeight: {
        0: '0',
        xl: '36rem',
      },
      boxShadow: {
        'outline-purple': '0 0 0 3px rgba(202, 191, 253, 0.45)',
        'outline-gray': '0 0 0 3px rgba(213,214,215,.45)',
      },
    },
    colors: {
      transparent: 'transparent',
      current: 'currentColor',
      black: black,
      white: white,
      green: green,
      purple: purple,
      orange: orange,
      red: red,
      teal: teal,
      gray: {
        50: '#f9fafb',
        100: '#f4f5f7',
        200: '#e5e7eb',
        300: '#d5d6d7',
        400: '#9e9e9e',
        500: '#707275',
        600: '#4c4f52',
        700: '#24262d',
        800: '#1a1c23',
        900: '#121317',
      },
    },
  },
  plugins: [],
}
