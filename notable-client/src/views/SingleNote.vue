<template>
  <div>
    <div class="page-container" v-if="state === 'loading'">
      <progress class="progress is-primary"></progress>
    </div>
    <card :title="title" v-if="state === 'viewing'">

    <section>
      <div class="content" v-html="body"></div>
    </section>

    <section class="action-buttons">
        <div class="field is-grouped">
          <div class="control">
            <a class="button"
               @click="edit"
               >
               Edit
            </a>
          </div>
          <div class="control">
            <a class="button is-danger"
               @click="state = 'confirm-delete'"
               >
               Delete
            </a>
          </div>
        </div>
    </section>

    </card>

    <div class="page-container" v-if="state === 'viewing'">
      <h1 class="title">Related Notes</h1>
      <div class="columns is-multiline">
        <div v-for="note in related"
          :key="note.id"
          class="column is-one-quarter">
          <mini-note
            :title="note.title"
            :body="note.body"
            @click="visitNote(note.id)">
          </mini-note>
        </div>
      </div>
    </div>

    <div class="modal"
         :class="{ 'is-active': state === 'confirm-delete' || state === 'deleting' }">
      <div class="modal-background"> </div>
      <div class="modal-card">
        <header class="modal-card-head">
          <h1 class="title">
            Confirm
          </h1>
        </header>
        <div class="modal-card-body">
          <h1 class="subtitle">Are you sure you want to delete this note?</h1>
        </div>
        <footer class="modal-card-foot">
          <div class="field is-grouped">
            <div class="control">
              <a @click="deleteNote"
                 class="button is-danger"
                 :class="{ 'is-loading': state === 'deleting' }">
                Delete
              </a>
            </div>
            <div class="control">
              <a @click="state = 'viewing'"
                 class="button">
                Cancel
              </a>
            </div>
          </div>
        </footer>
      </div>
    </div>

  </div>
</template>

<script>

import Card from '@/components/Card';
import MiniNote from "@/components/MiniNote.vue";

export default {
  name: 'single-note',
  components: { Card, MiniNote },
  data() {
    return {
      related: [],
      title: '',
      body: '',
      state: 'loading',
    }
  },
  mounted() {
    this.loadNote();
  },
  methods: {
    loadNote() {
      const id = this.$route.params.id;

      this.state = 'loading';

      this.axios.get(`/api/note/${id}`)
        .then(response => {
          response = response.data;
          this.title = response.title;
          this.body = this.showdown.makeHtml(response.body);
          this.state = 'viewing';
        })
        .catch();
      this.axios.get(`/api/note/${id}/similar?count=4`)
        .then(response => {
          this.related = response.data;
        })
    },

    edit() {
      this.$router.push({
        name: 'edit-note',
        params: { id: this.$route.params.id, }
      });
    },

    deleteNote() {
      this.state = 'deleting';

      const id = this.$route.params.id;
      this.axios.delete(`/api/note/${id}`)
        .then(() => this.$router.push({
          name: 'search'
        }));
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
      this.loadNote();
    }
  }
}
</script>

<style lang="scss">
@import "@/assets/styles.scss";

.action-buttons {
  padding-top: $gap;
}

</style>
