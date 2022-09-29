/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      screens: {
        tablet: '960px',
        desktop: '1248px',
      },
      // my monochromatic color schema
      colors: {
        white: '#FFFFFF',
        ink: '#060606',
        'ink-t1': '#676767',
        'ink-t2': '#81817E',
        'ink-t3': '#BFBEBE',
        'ink-t4': '#EEEEEE',
      },
      boxShadow: {
        sm: '0px 2px 4px 0px rgba(11, 10, 55, 0.15)',
        lg: '0px 8px 20px 0px rgba(18, 16, 99, 0.06)',
      },

      fontFamily: {
        roboto: ['Roboto', 'sans-serif'],
        montserrat: ['Montserrat', 'sans-serif'],
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/line-clamp'),
    require('@tailwindcss/typography'),
  ],
}
