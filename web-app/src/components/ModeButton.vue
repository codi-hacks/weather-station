<template>
  <v-btn @click="switchMode()" class="mode-button" elevation="0" x-small>
    <v-icon x-small>{{ icon(nextItem) }}</v-icon>
  </v-btn>
</template>

<script>
export default {
  props: {
    modes: {
      default: () => (['current', 'chart']),
      type: Array
    },
    value: {
      required: true,
      type: String
    }
  },
  computed: {
    nextItem() {
      const index = this.modes.findIndex(v => v === this.value)
      return this.modes[index + 1] || this.modes[0]
    }
  },
  methods: {
    icon(mode) {
      const map = {
        chart: 'mdi-chart-timeline-variant',
        current: 'mdi-counter',
        'percentage-chart': 'mdi-battery-60'
      }
      return map[mode]
    },
    switchMode() {
      this.$emit('input', this.nextItem)
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
