/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: 'jit',
  content: [
    "src/*.rs"
  ],
  theme: {
    extend: {
      backgroundImage: {
        'landing-page': "url('/landing_page_background.jpg')",
      },
      colors: {
        grey: '#353535',
        turquoise: '#34aca1',
        darkGreen: '#2d978d',
        lightGrey: '#e3e3e3',
        darkGrey: '#757575',
        hover: '#eeeeee',
        red: '#ff0000',
        green: 'rgb(0, 128, 0)',
        black26: 'rgba(0, 0, 0, 0.26)',
        black12: 'rgba(0, 0, 0, 0.12)'
      },
      fontFamily: {
        body: ['"Helvetica Neue"', "Helvetica", "Arial", "sans-serif"],
        header: ["Comfortaa"]
      },
      scale: {
        '200': '1.75'
      },
      blur: {
        xs: '2px'
      },
    }
  }
}

