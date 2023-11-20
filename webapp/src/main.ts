import { createApp } from 'vue'
import App from '~/App.vue'
import '~/style.css'
import Dashboard from '~/views/Dashboard.vue'
import Services from '~/views/Services.vue'
import NotFound from '~/views/404.vue'
import Login from '~/views/Login.vue'
import Page from '~/views/Page.vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import { createPinia } from 'pinia'

const routes = [
  {
    path: '/',
    component: Page,
    children: [
      {
        path: '',
        name: 'Dashboard',
        component: Dashboard,
        auth: true,
      },
      {
        path: 'services',
        name: 'Services',
        component: Services,
        auth: true,
      },
      {
        path: ':catchAll(.*)',
        component: NotFound,
        name: 'NotFound',
      },
    ],
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

/**
 * router.beforeEach(function (to, from, next) {
  console.log('beforeEach', to.path + ' - Auth: ' + auth.user.authenticated)
  if ((to.path !== '/login' && to.path !== 'login') && !auth.user.authenticated) {
    next({ path: '/login' })
  } else if ((to.path === '/login' || to.path === 'login') && auth.user.authenticated) {
    next({ path: '/' })
  } else {
    next()
  }
})
 */

const pinia = createPinia()

createApp(App).use(router).use(pinia).mount('#app')
