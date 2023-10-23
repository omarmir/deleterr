const colors = require('tailwindcss/colors')

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
            },
        },
        colors: {
            transparent: 'transparent',
            current: 'currentColor',
            black: colors.black,
            white: colors.white,
            emerald: colors.emerald,
            indigo: colors.indigo,
            yellow: colors.yellow,
            green: colors.green,
            purple: colors.purple,
            orange: colors.orange,
            red: colors.red,
            teal: colors.teal,
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
            'cool-gray': {
                50: '#fbfdfe',
                100: '#f1f5f9',
                200: '#e2e8f0',
                300: '#cfd8e3',
                400: '#97a6ba',
                500: '#64748b',
                600: '#475569',
                700: '#364152',
                800: '#27303f',
                900: '#1a202e',
            },
        },
    },
    plugins: [],
}
