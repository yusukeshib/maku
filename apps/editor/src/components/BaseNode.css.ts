import { style } from "@vanilla-extract/css";
import { theme } from "../theme.css";

export const nodeContainer = style({
  padding: 0,
  borderRadius: 0,
  background: theme.color.bg.secondary,
  minWidth: "200px",
  boxShadow: theme.shadow.md,
});

export const nodeHeader = style({
  display: "flex",
  alignItems: "center",
  gap: theme.spacing.sm,
  color: theme.color.text.primary,
  padding: theme.spacing.sm,
  background: theme.color.bg.tertiary,
});

export const nodeTitle = style({
  color: theme.color.text.primary,
});

export const nodeLabel = style({
  display: "none",
});

export const nodeAttrs = style({
  color: theme.color.text.tertiary,
  maxWidth: "200px",
  padding: theme.spacing.sm,
  borderRadius: 0,
});

export const handle = style({
  background: theme.color.port.default,
  width: 12,
  height: 12,
  borderRadius: "50%",
  border: `2px solid ${theme.color.bg.primary}`,
});
