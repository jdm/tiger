module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        plastic: {
          100: "#FFFFFF",
          200: "#D9E0EA",
          300: "#959AA9",
          400: "#555668",
          500: "#444655",
          600: "#30303A",
          700: "#272733",
          800: "#1E1E26",
          900: "#191921",
        }
      }
    },
  },
  plugins: [],
}