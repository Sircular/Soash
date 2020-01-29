<template>
  <div>
    <div class="field">
      <div class="control">
        <input
          class="input is-large"
          ref="titleField"
          :disabled="disabled"
          :value="value.title"
          @input="updateData()"/>
      </div>
    </div>
    <div class="editor-container">
      <vue-editor
        ref="editor"
        :disabled="disabled"
        :value="value.body"
        @input="updateData()" />
    </div>
  </div>
</template>

<script>

import { VueEditor } from 'vue2-editor';

export default {
  name: 'note-editor',
  props: {
    disabled: {
      type: Boolean,
      default: false,
    },
    value: {
      type: Object,
      required: true
    },
  },
  components: { VueEditor },
  methods: {
    updateData() {
      this.$emit('input', {
        title: this.$refs.titleField.value,
        body: this.$refs.editor.quill.getHTML(),
      });
    }
  }
}

</script>
