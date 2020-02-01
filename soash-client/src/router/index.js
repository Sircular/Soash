import Vue from 'vue';
import VueRouter from 'vue-router';

import Login from '@/views/Login.vue';
import Landing from '@/views/Landing.vue';
import Register from '@/views/Register.vue';
import NewNote from '@/views/NewNote.vue';
import Search from '@/views/Search.vue';
import SingleNote from '@/views/SingleNote.vue';
import EditNote from '@/views/EditNote.vue';

Vue.use(VueRouter)

export const routes = [
  {
    name: 'landing',
    path: '/',
    component: Landing,
  },
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
    name: 'register',
    path: '/register',
    component: Register
  },
  {
    name: 'new-note',
    path: '/note/new',
    component: NewNote,
  },
  {
    name: 'edit-note',
    path: '/note/:id/edit',
    component: EditNote,
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
