import { scan } from "react-scan";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App.tsx";

if (typeof window !== "undefined") {
  scan({
    enabled: true,
    log: false,
  });
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
