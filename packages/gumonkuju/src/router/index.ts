import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/error', component: () => import('../views/error.vue')},
    { path: '/:id', component: () => import('../views/view.vue'), props: true },
    { path: '/', component: () => import('../views/view.vue'), props: true },
  ],
})

export default router
