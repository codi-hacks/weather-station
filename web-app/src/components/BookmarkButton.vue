<template>
  <div class="bookmark-button">
    <v-btn x-small @click="checked = !checked">
      <v-icon v-if="checked">mdi-bookmark-check-outline</v-icon>
      <v-icon v-else>mdi-bookmark-plus</v-icon>
    </v-btn>
  </div>
</template>

<script>
export default {
  data() {
    return {
      checked: false
    }
  }
}
</script>

<style scoped>
</style>
