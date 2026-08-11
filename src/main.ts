import { mount } from 'svelte';
import './app.css';
import App from './App.svelte';

import init, { set_panic_hook, greet, add } from './lib/wasm/rust';
await init();
set_panic_hook();
try {
    greet('name');
    console.log(add(1, 2));
    console.log(add(2 ** 31 - 1, 0));
} catch (e) {
    console.error(e);
}

const app = mount(App, {
    target: document.getElementById('app')!,
});

export default app;
