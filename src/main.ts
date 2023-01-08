import { createPinia } from "pinia";
import { createApp } from "vue";
import { getState } from "./backend/api";
import App from "./App.vue";
import "./index.css";

createApp(App).use(createPinia()).mount("#app");

getState();
