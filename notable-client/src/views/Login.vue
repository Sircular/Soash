<template>
  <card title="Log In">
      <input-field
        ref="username"
        label="Username"
        v-model="username"
        @enter="focusRef('password')"
        :valid="state !== 'invalid' || !!username"
        :disabled="state === 'loading'"
        helptext="A username is required."
        />
      <input-field
        type="password"
        ref="password"
        label="Password"
        v-model="password"
        @enter="submit()"
        :valid="state !== 'invalid' || !!password"
        :disabled="state === 'loading'"
        helptext="A password is required."
        />

      <p v-if="state === 'failed'" class="has-text-danger">
      Invalid username or password
      </p>

      <a @click="submit"
         class="button is-primary"
         :class="{ 'is-loading': state === 'loading' }">
        Log In
      </a>
  </card>
</template>

<script>

import Card from '@/components/Card';
import InputField from '@/components/InputField';

export default {
  name: 'login',
  components: { Card, InputField },
  data() {
    return {
      username: '',
      password: '',
      state: 'unsubmitted'
    }
  },
  methods: {
    focusRef(name) {
      this.$refs[name].focus();
    },

    isFieldValid(name) {
      if (this.state !== 'invalid') { return true; }
      const elem = this.$refs[name];
      if (!elem) { return false; }
      return !!(elem.value);
    },

    submit() {
      this.state = 'validating';
      if (!(this.$refs.username.value && this.$refs.password.value)) {
        this.state = 'invalid';
      } else {
        this.state = 'loading';

        const params = new URLSearchParams();
        params.append('username', this.username);
        params.append('password', this.password);

        this.directAxios.post('/api/auth/login', params)
          .then(() => {
            this.state = 'successful';
            this.$root.$data.loggedIn = true;
            this.$router.push({ name: 'search' });
          })
          .catch(() => {
            this.state = 'failed';
            this.password = '';
          });
      }
    }
  }
}
</script>

<style lang="scss">
@import "@/assets/styles.scss";

.login-card {
  @extend .card;
  max-width: 500px;
  margin-left: auto;
  margin-right: auto;
  padding: $gap;
}

</style>
