<template>
  <ContentCard title="Log In">
  <div class="field">
    <label class="label">Username</label>
    <div class="control">
      <input class="input"
             :class="{ 'is-danger': !isFieldValid('username') }"
             ref="username"
             v-model="username"
             @keydown.enter="focusRef('password')"
             @focus="activeField = 'username'"
             @blur="activeField = ''"
             :disabled="state === 'loading'"
             />
      <p class="help is-danger" v-if="!isFieldValid('username')">
      A username is required.
      </p>
    </div>
  </div>

  <div class="field">
    <label class="label">Password</label>
    <div class="control">
      <input type="password"
             class="input"
             :class="{ 'is-danger': !isFieldValid('password') }"
             ref="password"
             v-model="password"
             @keydown.enter="submit()"
             @focus="activeField = 'password'"
             @blur="activeField = ''"
             :disabled="state === 'loading'"
             />
      <p class="help is-danger" v-if="!isFieldValid('password')">
      A password is required.
      </p>
    </div>
  </div>

  <div class="field">
    <div class="control">
      <a @click="submit"
         class="button is-primary"
         :class="{ 'is-loading': state === 'loading' }">Log In</a>
    </div>
  </div>

  <p v-if="state === 'failed'" class="has-text-danger">Invalid username or password</p>

  </ContentCard>
</template>

<script>

import axios from 'axios';
import ContentCard from '@/components/ContentCard';

export default {
  name: 'Login',
  components: { ContentCard },
  data() {
    return {
      username: '',
      password: '',
      activeField: '',
      state: 'unsubmitted'
    }
  },
  methods: {
    focusRef(name) {
      this.$refs[name].focus();
    },

    isFieldValid(name) {
      if (this.state !== 'missing') { return true; }
      const elem = this.$refs[name];
      if (!elem) { return false; }
      return !!(elem.value);
    },

    submit() {
      this.state = 'validating';
      if (!(this.$refs.username.value && this.$refs.password.value)) {
        this.state = 'missing';
      } else {
        this.state = 'loading';

        const params = new URLSearchParams();
        params.append('username', this.username);
        params.append('password', this.password);

        axios.post('/api/auth/login', params)
          .then(() => {
            this.state = 'successful';
            this.$router.push({ name: 'search' });
          })
          .catch(() => {
            this.state = 'failed';
            this.password = ''
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
