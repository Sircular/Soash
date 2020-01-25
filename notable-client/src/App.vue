<template>
  <div id="app">
    <Navbar :pages="pages"/>
    <div class="page-container">
      <router-view/>
    </div>
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
          title: "My Notes",
          path: "/search",
        }
      ]
    }
  },
}
</script>

<style lang="scss">
@import "@/assets/styles.scss";

.page-container {
  padding: $gap;
}

</style>
