import { createPinia } from "pinia";
import { createApp } from "vue";
import { getState } from "./api/app";
import App from "./App.vue";
import "./index.css";

createApp(App).use(createPinia()).mount("#app");

getState();
