<template>
  <div class="page-container">
    <div class="field has-addons">
      <div class="control is-expanded">
        <input class="input is-medium"
               ref="search-field"
               type="text"
               v-model="query"
               @keyup.enter="search"
               placeholders="Search Terms" />
      </div>
      <div class="control">
        <a class="button is-info is-medium"
           :class="{ 'is-loading': state === 'loading' }"
           @click="search">Search</a>
      </div>
    </div>
    <p class="has-text-centered"
       v-if="state === 'loaded' && query && !results.length">
    No notes found. :(<br/>
    <router-link to="/note/new">Create a new note</router-link>
    </p>
    <p class="has-text-centered" v-if="state === 'unsearched'">
    Enter some search terms to get started.
    </p>
    <div class="columns is-multiline">
      <div v-for="note in results" :key="note.id" class="column is-one-quarter">
        <mini-note
          :title="note.title"
          :body="note.body"
          @click="visitNote(note.id)">
        </mini-note>
      </div>
    </div>
  </div>
</template>

<script>
import MiniNote from "@/components/MiniNote.vue";

export default {
  name: 'home',
  components: {
    MiniNote
  },
  data() {
    return {
      state: 'unsearched',
      query: '',
      results: [],
    }
  },

  mounted() {
    this.query = this.$route.query.query;
    this.loadResults(this.$route.query.query);
  },

  methods: {

    search() {
      if (this.query) {
        this.$router.replace({
          name: 'search',
          query: { query: encodeURIComponent(this.query.trim()) }
        });
      } else {
        this.$router.replace({ name: 'search' });
        this.results = [];
        this.state = 'unsearched';
      }
    },

    loadResults() {
      if (this.query) {
        this.state = 'loading';
        this.axios.get('/api/note/search', {
          params: { query: this.query }
        })
          .then(response => {
            this.results = response.data;
            this.state = 'loaded';
          });
      }
    },

    visitNote(id) {
      this.$router.push({
        name: 'single-note',
        params: { id },
      });
    }

  },

  watch: {
    '$route' (_to, _from) {
      this.loadResults();
    }
  },
}
</script>
