import plugin from "tailwindcss/plugin";

/** @type {import('tailwindcss').Config} */
export default {
    darkMode: 'class',
    content: [
        "./index.html",
        "./pages/**/*.{vue,js,ts,jsx,tsx}",
        "./layouts/**/*.{vue,js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {},
    },
    plugins: [require('tailwindcss-primeui'), plugin(function ({matchUtilities, e}) {
        matchUtilities({
                'icon': (value) => ({
                    content: `"${value}"`,
                }),
            }
        )
    })]
}

