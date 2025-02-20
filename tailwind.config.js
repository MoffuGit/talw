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
          ".bg-base-300": {
            "background-color": "#dfdfe2",
          },
          ".border-base-300": {
            "border-color": "#dfdfe2"
          },
          ".border-r-base-300": {
            "border-right-color": "#dfdfe2"
          },
          ".border-l-base-300": {
            "border-left-color": "#dfdfe2"
          },
          ".border-t-base-300": {
            "border-top-color": "#dfdfe2"
          },
          ".border-b-base-300": {
            "border-bottom-color": "#dfdfe2"
          },
        },
        dark: {
          primary: "#B5BFE7",
          secondary: "#1f1f1f",
          accent: "#a4b0f0",
          neutral: "#222a2a",
          "base-100": "#27272a",
          "base-200": "#18181b",
          "base-300": "#09090b",
        },
      },
    ],
  },
  plugins: [require("daisyui"), require("tailwind-scrollbar")],
};
