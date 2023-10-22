/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')

export default {
    darkMode: 'class',
    content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
    theme: {
        extend: {
            fontFamily: {
                lato: 'lato, open-sans, sans-serif',
            },
            maxHeight: {
                0: '0',
                xl: '36rem',
            },
            boxShadow: {
                'outline-purple': '0 0 0 3px rgba(202, 191, 253, 0.45)',
            },
        },
    },
    plugins: [],
}
