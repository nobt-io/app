/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "src/*.rs"
  ],
  theme: {
    extend: {
      colors: {
        grey: '#353535',
        turquoise: '#34aca1',
        darkGreen: '#2d978d',
        lightGrey: '#e3e3e3',
        darkGrey: '#757575',
        red: '#ff0000',
        green: 'rgb(0, 128, 0)'
      },
      fontFamily: {
        body: ['"Helvetica Neue"', "Helvetica", "Arial", "sans-serif"],
        header: ["Comfortaa"]
      }
    }
  }
}

