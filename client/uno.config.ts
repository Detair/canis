import { defineConfig, presetUno, presetIcons } from "unocss";

export default defineConfig({
  presets: [
    presetUno(),
    presetIcons({
      scale: 1.2,
      cdn: "https://esm.sh/",
    }),
  ],
  theme: {
    colors: {
      primary: {
        DEFAULT: "#5865F2",
        hover: "#4752C4",
      },
      background: {
        primary: "#313338",
        secondary: "#2B2D31",
        tertiary: "#1E1F22",
      },
      text: {
        primary: "#F2F3F5",
        secondary: "#B5BAC1",
        muted: "#949BA4",
      },
      success: "#23A559",
      warning: "#F0B232",
      danger: "#DA373C",
    },
  },
  shortcuts: {
    "btn": "px-4 py-2 rounded-md font-medium transition-colors",
    "btn-primary": "btn bg-primary hover:bg-primary-hover text-white",
    "input-field": "w-full px-3 py-2 bg-background-tertiary rounded-md text-text-primary outline-none focus:ring-2 focus:ring-primary",
  },
});
