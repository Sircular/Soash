import Vue from 'vue';
import VueRouter from 'vue-router';

import Login from '@/views/Login.vue';
import NewNote from '@/views/NewNote.vue';
import Search from '@/views/Search.vue';
import SingleNote from '@/views/SingleNote.vue';

Vue.use(VueRouter)

export const routes = [
  {
    name: 'search',
    path: '/search',
    component: Search,
  },
  {
    name: 'login',
    path: '/login',
    component: Login,
  },
  {
    name: 'new-note',
    path: '/note/new',
    component: NewNote,
  },
  {
    name: 'single-note',
    path: '/note/:id',
    component: SingleNote,
  },
  {
    path: '*',
    component: Login,
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

export default router
