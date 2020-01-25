<template>
  <div class="navbar has-shadow has-background-white-ter">
    <div class="container">
      <div class="navbar-menu">
        <div class="navbar-end">
          <router-link
            class="navbar-item"
            exact-active-class="is-active"
            v-for="page in pages"
            :key="page.path"
            :to="page.path">
            {{ page.title }}
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Navbar',
  props: ['pages']
}
</script>

<style lang="scss">
@import "@/assets/styles.scss";

.navbar-item.is-active {
  @extend .has-background-white-bis;
}
</style>
