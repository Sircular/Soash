<template>
  <div id="app">
    <Navbar :pages="pages"/>
    <router-view/>
  </div>
</template>

<script>
import Navbar from '@/components/Navbar.vue';

export default {
  name: 'App',
  components: {
    Navbar,
  },
  data() {
    return {
      pages: [
        {
          title: 'My Notes',
          path: '/search',
        },
        {
          title: 'New Note',
          path: '/note/new',
        }
      ]
    }
  },
  mounted() {
    this.directAxios.get('/api/auth/verify')
      .then(() => {
        this.$root.$data.loggedIn = true;
      })
      .catch(() => this.$root.$data.loggedIn = false);
  },
}
</script>
