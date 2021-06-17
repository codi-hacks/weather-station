<template>
  <div class="time-button">
    <v-btn-toggle mandatory
      :value="value" @change="updateValue($event)">
      <v-btn x-small :value="846e5">1d</v-btn>
      <v-btn x-small :value="1728e5">2d</v-btn>
      <v-btn x-small :value="6048e5">1w</v-btn>
      <!-- Displaying all measurements is equivalent to 90 days
        since the server limits the time series to 90 days -->
      <v-btn x-small :value="Infinity">90d</v-btn>
    </v-btn-toggle>
  </div>
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
.time-button {
  margin-top: 2px;
  position: absolute;
  left: 30%;
  right: 30%;
  margin-left: auto;
  margin-right: auto;
  z-index: 1;
}

v-btn-toggle {
  width:100%;
  margin-left: auto;
  margin-right: auto;
}
</style>>
