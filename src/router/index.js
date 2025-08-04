import { createRouter, createWebHistory } from 'vue-router'



const routes = [
  {
    path: '/',
    redirect: '/login'
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('@/views/LoginView.vue'),
  },
  {
    path: '/mainLayout',
    name: 'mainLayout',
    component: () => import('@/views/MainLayout.vue'),
    children: [
      {
        path: 'home',
        name: 'home',
        component: () => import('@/views/contentViews/HomeView.vue'),
      },
      {
        path: 'translate',
        name: 'translate',
        component: () => import('@/views/contentViews/TranslateView.vue'),
      },
      {
        path: 'tido',
        name: 'tido',
        component: () => import('@/views/contentViews/TidoView.vue'),
      },
      {
        path: 'ai',
        name: 'ai',
        component: () => import('@/views/contentViews/AiView.vue'),
      },
      {
        path: 'codeFeeder',
        name: 'codeFeeder',
        component: () => import('@/views/contentViews/CodeFeederView.vue'),
      },
      {
        path: 'getByBilibili',
        name: 'getByBilibili',
        component: () => import('@/views/contentViews/GetByBilibili.vue'),
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})


export default router