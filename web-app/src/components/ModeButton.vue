<template>
  <v-btn
    @click="switchMode()"
    class="v-btn-toggle mode-button"
    elevation="0"
    outlined
    x-small
  >
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
  border-color: var(--v-secondary-lighten5);
  margin-top: 2px;
  position: absolute;
  left: 2px;
  z-index: 1;
}
</style>
