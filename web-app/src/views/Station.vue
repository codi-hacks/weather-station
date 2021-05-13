<template>
  <div>
    {{ params }}
  </div>
</template>

<script>
export default {
  name: 'Station',
  data() {
    return {
    }
  },
  computed: {
    params() {
      return this.$route.params
    }
  }
}
</script>
