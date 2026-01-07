/* @refresh reload */
import { render } from "solid-js/web";
import { Router } from "@solidjs/router";
import "virtual:uno.css";
import "@unocss/reset/tailwind.css";
import "./styles/global.css";
import App from "./App";

const root = document.getElementById("root");

if (root) {
  render(
    () => (
      <Router>
        <App />
      </Router>
    ),
    root
  );
}
