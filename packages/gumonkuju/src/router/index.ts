import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: () => import('../views/view.vue') },
    { path: '/:view', component: () => import('../views/view.vue') },
  ],
})

export default router