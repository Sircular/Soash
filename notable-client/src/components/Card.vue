<template>
  <div class="page-container">
    <div class="card">
      <div class="card-header" v-if="!!title">
        <h1 class="card-header-title title">{{ title }}</h1>
      </div>
      <div class="card-content">
        <slot></slot>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'card',
  props: ['title'],
}
</script>

<style lang="scss">
@import "@/assets/styles.scss";

</style>
