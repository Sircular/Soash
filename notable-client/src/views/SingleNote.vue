<template>
  <content-card :title="title">
    <span v-html="body"></span>
  </content-card>
</template>

<script>

import ContentCard from '@/components/ContentCard';

export default {
  name: 'single-note',
  components: { ContentCard },
  data() {
    return {
      title: '',
      body: '',
      state: 'loading',
    }
  },
  mounted() {
    const id = this.$route.params.id;
    this.axios.get(`/api/note/${id}`)
      .then(response => {
        response = response.data;
        this.title = response.title;
        this.body = response.body;
      })
      .catch();
    this.state = 'loaded';
  }
}
</script>

<style lang="scss">
@import "@/assets/styles.scss";

.login-card {
  @extend .card;
  max-width: 500px;
  margin-left: auto;
  margin-right: auto;
  padding: $gap;
}

</style>
