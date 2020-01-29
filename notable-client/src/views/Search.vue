<template>
  <div class="container">
    <div class="field has-addons">
      <div class="control is-expanded">
        <input class="input is-medium"
               ref="search-field"
               type="text"
               @keyup.enter="search"
               placeholders="Search Terms" />
      </div>
      <div class="control">
        <a class="button is-info is-medium" @click="search">Search</a>
      </div>
    </div>
    <FullNote />
  </div>
</template>

<script>
import FullNote from "@/components/FullNote.vue";

export default {
  name: 'home',
  components: {
    FullNote,
  },

  mounted() {
    this.loadResults(this.$route.query.query);
  },

  methods: {

    search() {
      try {
        let query = this.$refs['search-field'].value;
        if (query) {
          this.$router.replace({
            name: 'search',
            query: { query: encodeURIComponent(query.trim()) }
          });
        } else {
          this.$router.replace({ name: 'search' });
        }
      } catch(e) {
        console.log(e);
      }
    },

    loadResults(query) {
      if (query) {
        this.axios.get('/api/note/search', {
          params: { query: query }
        })
          .then(response =>
            console.log(response)
          );
      }
    },

  },

  watch: {
    '$route' (to, _from) {
      this.loadResults(to.query.query);
    }
  },
}
</script>
