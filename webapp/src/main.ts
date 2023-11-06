import { createApp } from 'vue'
import App from '~/App.vue'
import '~/style.css'
import Dashboard from '~/views/Dashboard.vue'
import Services from '~/views/Services.vue'
import NotFound from '~/views/404.vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import { createPinia } from 'pinia'

const routes = [
  { path: '/services', component: Services },
  { path: '/', component: Dashboard },
  {
    path: '/:catchAll(.*)',
    component: NotFound,
    name: 'NotFound',
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

const pinia = createPinia()

createApp(App).use(router).use(pinia).mount('#app')
