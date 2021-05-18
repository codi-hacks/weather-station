<template>
  <v-btn @click="toggleMode()" class="mode-button" elevation="0" x-small>
    <v-icon x-small v-if="value === 'current'">mdi-chart-timeline-variant</v-icon>
    <v-icon x-small v-if="value === 'chart'">mdi-counter</v-icon>
  </v-btn>
</template>

<script>
export default {
  props: {
    value: {
      required: true,
      type: String
    }
  },
  methods: {
    toggleMode() {
      this.$emit('input', this.value === 'current' ? 'chart' : 'current')
    }
  }
}
</script>

<style scoped>
.mode-button {
  margin-top: 2px;
  position: absolute;
  left: 0;
  z-index: 1;
}
</style>
