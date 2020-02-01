<template>
  <div>
    <card title="Register">
    <input-field
      ref="username"
      label="Username"
      v-model="username"
      @enter="focusRef('password')"
      :valid="state !== 'invalid' || !!username"
      :disabled="state === 'loading'"
      helptext="A username is required." />
    <input-field
      type="password"
      ref="password"
      label="Password"
      v-model="password"
      @enter="focusRef('confirmPassword')"
      :valid="state !== 'invalid' || !!password"
      :disabled="state === 'loading'"
      helptext="A password is required." />
    <input-field
      type="password"
      ref="confirmPassword"
      label="Confirm Password"
      v-model="confirmPassword"
      @enter="submit"
      :valid="state !== 'invalid' || password === confirmPassword"
      :disabled="state === 'loading'"
      helptext="Passwords must match." />

    <p v-if="state === 'failed'" class="has-text-danger">
    That username is taken.
    </p>

    <a @click="submit"
       class="button is-primary"
       :class="{ 'is-loading': state === 'loading' }"
       >
       Register
    </a>
    </card>
    <section class="page-container">
      <p>
      Already have an account?
      <router-link to="/login">Log in here.</router-link>
      </p>
    </section>
  </div>
</template>

<script>

import Card from '@/components/Card';
import InputField from '@/components/InputField';

export default {
  name: 'register',
  components: { Card, InputField },
  data() {
    return {
      username: '',
      password: '',
      confirmPassword: '',
      state: 'unsubmitted',
    };
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

      if (!(this.$refs.username.value &&
        this.$refs.password.value === this.$refs.confirmPassword.value)) {
        this.state = 'invalid';
      } else {
        const params = new URLSearchParams();
        params.append('username', this.username);
        params.append('password', this.password);

        this.directAxios.post('api/auth/register', params)
          .then(() => {
            this.state = 'successful';
            this.$router.push({ name: 'login' })
          })
          .catch(() => {
            this.state = 'failed';
            this.username = '';
            this.password = '';
            this.confirmPassword = '';
            this.$refs.username.focus();
          } );
      }
    }

  }
}

</script>
