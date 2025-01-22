/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  darkMode: "class",
  theme: {
    extend: {
      fontFamily: {
        geist: ["Geist", "sans-serif"],
        satoshi: ["Satoshi", "sans-serif"],
      },
      transitionProperty: {
        height:
          "height, color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
        border:
          "border-radius"
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
          ".bg-base-400": {
            "background-color": "#dfdfe2",
          },
          ".border-base-400": {
            "border-color": "#dfdfe2"
          },
          ".border-r-base-400": {
            "border-right-color": "#dfdfe2"
          },
          ".border-l-base-400": {
            "border-left-color": "#dfdfe2"
          },
          ".border-t-base-400": {
            "border-top-color": "#dfdfe2"
          },
          ".border-b-base-400": {
            "border-bottom-color": "#dfdfe2"
          },
        },
        dark: {
          primary: "#B5BFE7",
          secondary: "#1f1f1f",
          accent: "#a4b0f0",
          neutral: "#222a2a",
          "base-100": "#2c2d31",
          "base-200": "#27282b",
          "base-300": "#1c1c1f",
          ".bg-base-400": {
            "background-color": "#0d0d0d"
          },
          ".border-base-400": {
            "border-color": "#0d0d0d"
          },
          ".border-r-base-400": {
            "border-right-color": "#0d0d0d"
          },
          ".border-l-base-400": {
            "border-left-color": "#0d0d0d"
          },
          ".border-t-base-400": {
            "border-top-color": "#0d0d0d"
          },
          ".border-b-base-400": {
            "border-bottom-color": "#0d0d0d"
          },
        },
      },
    ],
  },
  plugins: [require("daisyui"), require("tailwind-scrollbar")],
};
