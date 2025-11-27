import { style } from "@vanilla-extract/css";
import { theme } from "../theme.css";

export const paletteContainer = style({
  position: "absolute",
  top: 10,
  left: 10,
  background: theme.color.bg.secondary,
  borderRadius: theme.borderRadius.lg,
  boxShadow: theme.shadow.lg,
  padding: theme.spacing.lg,
  maxWidth: "250px",
  maxHeight: "calc(100vh - 40px)",
  overflowY: "auto",
  zIndex: 10,
  border: `1px solid ${theme.color.border.primary}`,
});

export const categoryContainer = style({
  marginBottom: theme.spacing.lg,
});

export const categoryTitle = style({
  color: theme.color.text.secondary,
  marginBottom: theme.spacing.sm,
  textTransform: "uppercase",
  letterSpacing: "0.5px",
});

export const buttonList = style({
  display: "flex",
  flexDirection: "column",
  gap: theme.spacing.xs,
});

export const nodeButton = style({
  padding: "8px 12px",
  borderRadius: theme.borderRadius.sm,
  background: theme.color.bg.tertiary,
  cursor: "pointer",
  display: "flex",
  alignItems: "center",
  gap: theme.spacing.sm,
  border: "none",
  color: theme.color.text.primary,
  ":hover": {
    background: theme.color.bg.hover,
  },
});

export const nodeButtonText = style({
  color: theme.color.text.primary,
});
