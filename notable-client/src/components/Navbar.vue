<template>
  <div class="navbar has-shadow has-background-white-ter">
    <div class="container">

      <div class="navbar-brand">
        <div class="navbar-item">
          <router-link to="/">
            <img src="/img/logo-dark.svg" alt="soash logo"/>
          </router-link>
        </div>
        <a role="button"
          class="navbar-burger burger"
          :aria-expanded="expanded"
          :class="{ 'is-active': expanded }"
          @click="expanded = !expanded"
          >
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>

      <div class="navbar-menu"
           :class="{ 'is-active': expanded }" >
        <div class="navbar-start">
          <router-link
            class="navbar-item"
            exact-active-class="is-active"
            v-for="page in leftPages"
            :key="page.path"
            :to="page.path">
            {{ page.title }}
          </router-link>
        </div>
        <div class="navbar-end">
          <div class="navbar-item" v-if="loggedIn === false">
            <router-link
              to="/register"
              class="button is-primary" >
              Sign Up
            </router-link>
          </div>
          <div class="navbar-item" v-if="loggedIn === false">
            <router-link
              to="/login"
              class="button">
              Log In
            </router-link>
          </div>
          <div class="navbar-item" v-if="loggedIn === true">
            <a @click="logOut" class="button">
              Log Out
            </a>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script>
export default {
  name: 'Navbar',
  props: ['pages'],
  data() {
    return {
      expanded: false,
    };
  },
  computed: {
    leftPages() {
      if (this.loggedIn === true) {
        return this.pages;
      } else {
        return [];
      }
    },
    loggedIn() {
      return !!this.$root.$data.loggedIn;
    }
  },
  methods: {
    logOut() {
      this.directAxios.get('api/auth/logout')
        .then(() => {
          this.$root.$data.loggedIn = false;
          this.$router.push({ name: 'login' });
        })
    }
  }
}
</script>

<style lang="scss">
@import "@/assets/styles.scss";

.navbar-item.is-active {
  @extend .has-background-white-bis;
}
</style>
