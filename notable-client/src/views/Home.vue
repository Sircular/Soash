<template>
  <div class="container">
    wheep
    <FullNote />
  </div>
</template>

<script>
import FullNote from "@/components/FullNote.vue";

export default {
  name: 'home',
  components: {
    FullNote,
  },
  mounted() {
    fetch('/api/foobar/1').then(r => console.log(r)).catch(e => console.log(e));
  },
}
</script>
