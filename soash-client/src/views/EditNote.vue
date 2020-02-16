<template>
  <card>
  <div class="editor-container">
    <note-editor v-model="note" :disabled="state !== 'editing'"/>
  </div>
  <div class="field is-grouped">
    <div class="control">
      <a @click="submit"
         class="button is-primary"
         :class="{ 'is-loading': state === 'saving' }"
         :disabled="state === 'loading'">
        Save
      </a>
    </div>
    <div class="control">
      <a @click="cancel"
         class="button"
         :disabled="state === 'saving'">
        Cancel
      </a>
    </div>
  </div>
  </card>
</template>

<script>

import Card from '@/components/Card';
import NoteEditor from '@/components/NoteEditor';

export default {
  name: 'edit-note',
  components: { Card, NoteEditor },
  data() {
    return {
      state: 'loading',
      note: {
        title: '',
        body: '',
      }
    }
  },
  mounted() {
    const id = this.$route.params.id;
    this.axios.get(`/api/note/${id}`)
      .then(response => {
        response = response.data;
        this.note.title = response.title;
        this.note.body = this.showdown.makeHtml(response.body);
        this.state = 'editing';
      })
      .catch();
  },
  methods: {
    submit() {
      this.state = 'saving';

      let id = this.$route.params.id;
      let note = {
        title: this.note.title,
        body: this.note.body,
      };

      this.axios.post(`/api/note/${id}/update`, note)
        .then(() => {
          this.$router.push({
            name: 'single-note',
            params: { id: this.$route.params.id },
          });
        })
    },
    cancel() {
      this.$router.push({
        name: 'single-note',
        params: { id: this.$route.params.id },
      });
    },
  }
}

</script>

<style lang="scss">
@import "@/assets/styles.scss";

.editor-container {
 padding-bottom: $gap / 4;
}

</style>
