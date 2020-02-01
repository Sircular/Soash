import Vue from 'vue'
import VueCookie from 'vue-cookie';
import showdown from 'showdown';
import axios from 'axios';

import App from './App.vue';
import router from './router'

Vue.use(VueCookie);

const directAxiosInstance = axios.create();
Vue.mixin({
  data: function() {
    return {
      get axios() {
        return axiosInstance;
      },
      get directAxios() {
        return directAxiosInstance;
      },
    }
  }
});

// Create a global axios instance for error handling purposes
const axiosInstance = axios.create();
axiosInstance.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response.status === 401) {
      router.push({ name: 'login' });
    }
    return Promise.reject(error);
  }
);

// Create a global instance of the Showdown converter
const converter = new showdown.Converter();
Vue.mixin({
  data: function() {
    return {
      get showdown() {
        return converter;
      }
    }
  }
})

Vue.config.productionTip = false;

new Vue({
  data: {
    loggedIn: null,
  },
  router,
  render: h => h(App)
}).$mount('#app');
