import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: "class", // 👈 required for next-themes
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};

export default config;
