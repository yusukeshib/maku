import { createGlobalTheme, globalStyle } from "@vanilla-extract/css";

export const theme = createGlobalTheme(":root", {
  color: {
    // Background colors
    bg: {
      primary: "#ffffff",
      secondary: "#f5f5f5",
      tertiary: "#e5e5e5",
      hover: "#dcdcdc",
    },
    // Text colors
    text: {
      primary: "#1a1a1a",
      secondary: "#4a4a4a",
      tertiary: "#6a6a6a",
    },
    // Border colors
    border: {
      primary: "#d0d0d0",
      secondary: "#c0c0c0",
      hover: "#a0a0a0",
    },
    // Node colors (vibrant colors for handles/ports)
    port: {
      default: "#f59e0b",
      input: "#10b981",
      output: "#3b82f6",
    },
    // Edge colors
    edge: {
      default: "#999999",
      selected: "#3b82f6",
    },
  },
  spacing: {
    xs: "4px",
    sm: "8px",
    md: "12px",
    lg: "16px",
    xl: "24px",
  },
  borderRadius: {
    sm: "4px",
    md: "8px",
    lg: "12px",
  },
  shadow: {
    sm: "0 1px 3px rgba(0, 0, 0, 0.1)",
    md: "0 4px 6px rgba(0, 0, 0, 0.15)",
    lg: "0 10px 15px rgba(0, 0, 0, 0.2)",
  },
  fontFamily: {
    sans: "sans-serif",
    mono: "monospace",
  },
  fontSize: {
    xs: "11px",
    sm: "12px",
    base: "14px",
    md: "16px",
    lg: "18px",
    xl: "20px",
    "2xl": "24px",
    "3xl": "30px",
  },
  fontWeight: {
    normal: "400",
    medium: "500",
    semibold: "600",
    bold: "700",
  },
  lineHeight: {
    tight: "1.25",
    normal: "1.5",
    relaxed: "1.75",
  },
});

// Global styles
globalStyle("html, body", {
  margin: 0,
  padding: 0,
  fontFamily: theme.fontFamily.sans,
  fontSize: theme.fontSize.base,
  fontWeight: theme.fontWeight.normal,
  lineHeight: theme.lineHeight.normal,
  color: theme.color.text.primary,
  backgroundColor: theme.color.bg.primary,
});

globalStyle("*, *::before, *::after", {
  boxSizing: "border-box",
});
