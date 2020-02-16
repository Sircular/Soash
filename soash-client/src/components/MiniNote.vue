<template>
  <div class="mini-card">
    <div class="card-content" @click="$emit('click')">
      <div class="hidden-shadow"></div>
      <strong>{{ title }}</strong>
      <span v-html="body"></span>
    </div>
  </div>
</template>

<script>

export default {
  name: 'mini-note',
  props: {
    title: String,
    body: String,
  },
}

</script>

<style lang="scss">
@import "@/assets/styles.scss";

.mini-card {
  @extend .card;
  max-height: 200px;
  overflow: hidden;
  cursor: pointer;
}

$shadow-color: white;

.hidden-shadow {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: linear-gradient(transparent 65%, $shadow-color 85%);
}

</style>
