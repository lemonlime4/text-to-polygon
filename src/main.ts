import init, { set_panic_hook } from "./lib/wasm/rust";
await init();
set_panic_hook();

import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
