import { createApp } from 'vue'
import App from './App.vue'
import './style.css'
import Dashboard from './views/Dashboard.vue'
import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [{ path: '/', component: Dashboard }]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

createApp(App).use(router).mount('#app')
