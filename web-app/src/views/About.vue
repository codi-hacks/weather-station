<template>
  <v-container>
    <div class="about">
      <h1>This is an about page</h1>
    </div>
  </v-container>
</template>

<script>
export default {
  created() {
    this.$store.commit('setPageTitle', 'About')
  }
}
</script>
