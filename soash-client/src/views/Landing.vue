<template>
  <div class="hero is-primary is-fullheight">
    <div class="hero-body">
      <div class="container">
        <img src="/img/logo-light.svg" alt="soash." style="height: 4em"/>
        <h1 class="title">
          The free-associative personal search engine.
        </h1>
        <h3 class="subtitle">BOTTOM TEXT</h3>
        <router-link to="/register"
          class="button is-large is-primary is-outlined is-inverted">
          Sign Up
        </router-link>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'landing',
  computed: {
    loggedIn() {
      return !!this.$root.$data.loggedIn;
    }
  },
  mounted() {
    if (this.loggedIn) {
      this.$router.push({ name: 'search' });
    }
  },
  watch: {
    loggedIn(to, _from) {
      if (to) {
        this.$router.push({ name: 'search' });
      }
    }
  }
}
</script>
