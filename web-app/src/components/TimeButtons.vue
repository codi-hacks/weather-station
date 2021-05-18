<template>
  <v-btn-toggle class="time-buttons" mandatory :value="value" @change="updateValue($event)">
    <v-btn x-small :value="846e5">1d</v-btn>
    <v-btn x-small :value="1728e5">2d</v-btn>
    <v-btn x-small :value="4536e5">1w</v-btn>
  </v-btn-toggle>
</template>

<script>
export default {
  props: {
    value: {
      required: true,
      type: Number
    }
  },
  methods: {
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
