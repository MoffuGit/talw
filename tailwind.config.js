/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  darkMode: "class",
  theme: {
    fontWeight: {
      black: "900",
      bold: "700",
      medium: "500",
      normal: "400",
      light: "300",
    },
    extend: {
      fontFamily: {
        satoshi: ["Satoshi", "sans-serif"],
      },
      transitionProperty: {
        height:
          "height, color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
      },
      boxShadow: {
        radio: " 0 0 0 4px theme(colors.base-100) inset",
      },
      keyframes: {
        "tooltip-open": {
          from: {
            opacity: "0",
          },
          to: {
            opacity: " 100%",
          },
        },
      },
      animation: {
        "tooltip-open": "tooltip-open 0.1s linear 1",
      },
    },
  },
  daisyui: {
    themes: [
      {
        light: {
          primary: "#b1b1ce",
          secondary: "#dadaec",
          accent: "#ad9497",
          neutral: "#222a2a",
          "base-100": "#f6f6f8",
          "base-200": "#eeeef0",
          "base-300": "#e9e9ec",
        },
        dark: {
          primary: "#B5BFE7",
          secondary: "#1f1f1f",
          accent: "#a4b0f0",
          neutral: "#222a2a",
          "base-100": "#2c2d31",
          "base-200": "#27282b",
          "base-300": "#1c1c1f",
        },
      },
    ],
  },
  plugins: [require("daisyui"), require("tailwind-scrollbar")],
};
