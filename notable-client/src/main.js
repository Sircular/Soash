import Vue from 'vue'
import VueCookie from 'vue-cookie';
import showdown from 'showdown';
import axios from 'axios';

import App from './App.vue';
import router from './router'

Vue.use(VueCookie);

// Create a global axios instance for configuration purposes
const axiosInstance = axios.create();
Vue.mixin({
  data: function() {
    return {
      get axios() {
        return axiosInstance;
      }
    }
  }
})

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
  router,
  render: h => h(App)
}).$mount('#app');
