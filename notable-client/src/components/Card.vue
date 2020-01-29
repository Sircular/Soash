<template>
  <div class="content-card">
    <slot></slot>
  </div>
</template>

<script>
export default {
  name: 'card',
}
</script>

<style lang="scss">
@import "@/assets/styles.scss";

.content-card {
  @extend .card;
  margin-left: auto;
  margin-right: auto;
  padding: $gap;
  max-width: 960px;
}

</style>
