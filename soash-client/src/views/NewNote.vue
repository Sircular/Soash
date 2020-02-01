<template>
  <card>
    <div class="editor-container">
      <note-editor v-model="note" :disabled="state !== 'editing'"/>
    </div>
    <div class="field">
      <div class="control">
        <a @click="submit"
           class="button is-primary"
           :class="{ 'is-loading': state === 'saving' }">
          Save
        </a>
      </div>
    </div>
  </card>
</template>

<script>

import Card from '@/components/Card';
import NoteEditor from '@/components/NoteEditor';

export default {
  name: 'new-note',
  components: { Card, NoteEditor },
  data() {
    return {
      note: {
        title: '',
        body: '',
      },
      state: 'editing',
    }
  },
  methods: {
    submit() {
      this.state = 'saving';

      let note = {
        title: this.note.title,
        body: this.note.body,
      };
      note.body = this.showdown.makeMarkdown(note.body);

      this.axios.post('/api/note/new', note)
        .then((response) => {
          const noteId = response.data;
          this.$router.push({
            name: 'single-note',
            params: {
              id: noteId
            },
          });
        });
    }
  },
}

</script>

<style lang="scss">
@import "@/assets/styles.scss";

.editor-container {
 padding-bottom: $gap / 4;
}

</style>
