<template>
  <div class="time-buttons">
    <v-btn x-small v-if="zoomedIn" @click="resetZoom()">Reset</v-btn>
    <v-btn-toggle mandatory :value="value" @change="updateValue($event)">
      <v-btn x-small :value="846e5">1d</v-btn>
      <v-btn x-small :value="1728e5">2d</v-btn>
      <v-btn x-small :value="6048e5">1w</v-btn>
      <v-btn x-small :value="7776e6">90d</v-btn>
    </v-btn-toggle>
  </div>
</template>

<script>
export default {
  props: {
    value: {
      required: true,
      type: Number
    },
    zoomedIn: {
      required: true,
      type: Boolean
    }
  },
  methods: {
    resetZoom() {
      this.$emit('reset-zoom')
    },
    updateValue(newValue) {
      this.$emit('input', newValue)
    }
  }
}
</script>

<style scoped>
.time-buttons {
  margin-top: 2px;
  position: absolute;
  right: 0;
  z-index: 1;
}
</style>
