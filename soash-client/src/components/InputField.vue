<template>
  <div class="field">
    <label class="label">{{ label }}</label>
    <div class="control">
      <input
        ref="fieldInput"
        :type="type"
        class="input"
        :class="{ 'is-danger': !valid }"
        @keydown.enter="$emit('enter')"
        @focus="$emit('focus')"
        @blur="$emit('blur')"
        :disabled="disabled"
        :value="value"
        @input="updateValue"
        />
      <p class="help is-danger" v-if="!valid">{{ helptext }}</p>
    </div>
  </div>
</template>

<script>

export default {
  name: 'input-field',
  props: {
    label: String,
    value: String,
    valid: {
      type: Boolean,
      default: true,
    },
    type: {
      type: String,
      default: 'input',
    },
    disabled: Boolean,
    helptext: String,
  },
  methods: {
    focus() {
      this.$refs.fieldInput.focus();
    },
    updateValue() {
      this.$emit('input', this.$refs.fieldInput.value);
    }
  }
}

</script>
