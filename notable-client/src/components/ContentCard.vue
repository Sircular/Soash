<template>
  <card>
    <h1 class="title">{{ title }}</h1>
    <div class="card-body">
      <slot></slot>
    </div>
  </card>
</template>

<script>

import Card from '@/components/Card.vue';

export default {
  name: 'content-card',
  components: { Card },
  props: {
    title: String,
  },
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

.card-body {
  margin-top: 12px;
}

</style>
