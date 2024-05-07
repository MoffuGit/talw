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
          accent: "#575785",
          neutral: "#222a2a",
          "base-100": "#f4f6f6",
          "base-200": "#f1f4f4",
          "base-300": "#e0e6e6",
        },
        dark: {
          primary: "#aab5b5",
          secondary: "#1f1f1f",
          accent: "#83a0a0",
          neutral: "#222a2a",
          "base-100": "#151515",
          "base-200": "#0e0e0e",
          "base-300": "#080808",
        },
      },
    ],
  },
  plugins: [require("daisyui"), require("tailwind-scrollbar")],
};
