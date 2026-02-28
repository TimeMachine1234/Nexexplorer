import App from "./App.svelte";
import "./styles/app.css";
import { mount } from "svelte";

// Prevent WebView2 built-in page zoom on Ctrl+wheel / trackpad pinch.
// zoomHotkeysEnabled is true so pinch gestures reach JS as ctrlKey:true wheel events,
// but we block the default page-zoom behavior here.
document.addEventListener("wheel", (e) => {
  if (e.ctrlKey) e.preventDefault();
}, { passive: false });

// Also block Ctrl+Plus/Minus/0 keyboard zoom
document.addEventListener("keydown", (e) => {
  if (e.ctrlKey && (e.key === "+" || e.key === "-" || e.key === "=" || e.key === "0")) {
    e.preventDefault();
  }
});

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
