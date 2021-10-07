<template>
  <div class="container">
    <Graph
      v-if="measurements.length"
      class="sparkline"
      :measurements="measurements"
      :options="chartOptions"
    />
    <ul class="text-body">
      <li></li>
      <li>
        <div class="header">
          <slot name="header1">Current</slot>
        </div>
        <div class="value">
          <slot name="value1">Error</slot>
        </div>
      </li>
      <li v-if="$slots.value2">
        <div class="header">
          <slot name="header2">Average</slot>
        </div>
        <div class="value">
          <slot name="value2">Error</slot>
        </div>
      </li>
      <li v-if="$slots.value3">
        <div class="header">
          <slot name="header3">Header3</slot>
        </div>
        <div class="value">
          <slot name="value3">Error</slot>
        </div>
      </li>
      <li></li>
    </ul>
  </div>
</template>

<script>
import Graph from './Graph'

export default {
  components: {
    Graph
  },
  props: {
    measurements: {
      required: false,
      type: Array,
      default: () => ([])
    }
  },
  computed: {
    chartOptions() {
      return {
        chart: {
          sparkline: {
            enabled: true
          }
        },
        grid: {
          row: {
            opacity: 0
          }
        },
        stroke: {
          curve: 'smooth',
          width: 3
        }
      }
    }
  }
}
</script>

<style scoped>
.container {
  height: 100%;
  z-index: 0;
}

.text-body {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  line-height: 1.1;
  padding-top: 4px;
  position: relative;
  z-index: 1;
}

.text-body li {
  align-content: center;
  display: flex;
  flex-basis: 100%;
  flex-direction: column;
  justify-content: center;
  padding-bottom: 4px;
  padding-top: 4px;
  text-align: center;
  text-shadow: 2px 2px var(--v-text-inverse-base);
}

.header {
  font-size: 2.5em;
}

.sparkline {
  padding-top: 4px;
  position: absolute;
  z-index: 0;
}

.value {
  font-size: 3.5em;
}

@media screen and (max-width: 640px) {
  .text-body li {
    flex-basis: 50%;
    flex-direction: column wrap;
    flex-flow: column;
    flex-grow: 1;
    justify-content: center;
    text-align: center;
  }

  .text-body li:first-child {
    display: none;
  }

  .text-body li:last-child {
    display: none;
  }

  .header {
    flex-basis: 50%;
    /*flex-direction: column;*/
    flex-flow: row wrap;
    font-size: 1.75em;
  }

  .value {
    flex-basis: 50%;
    /*flex-direction: column;*/
    flex-flow: row wrap;
    font-size: 1.75em;
  }
}

</style>
