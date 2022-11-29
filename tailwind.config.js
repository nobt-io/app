/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "src/*.rs"
  ],
  theme: {
    extend: {
      colors: {
        grey: '#353535',
        green: '#34aca1',
        darkGreen: '#2d978d',
        lightGrey: '#e3e3e3',
        darkGrey: '#757575',
        red: '#ff0000'
      },
      fontFamily: {
        body: ['"Helvetica Neue"', "Helvetica", "Arial", "sans-serif"],
        header: ["Comfortaa"]
      }
    }
  }
}

