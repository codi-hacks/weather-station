<template>
  <VueApexCharts
    class="chart"
    ref="chart"
    height="100%"
    :type="chartType"
    :options="chartOptions"
    :series="series"
    />
</template>

<script>
import objectAssignDeep from 'object-assign-deep'
import VueApexCharts from 'vue-apexcharts'

export default {
  components: {
    VueApexCharts
  },
  props: {
    chartType: {
      required: false,
      type: String,
      default: 'line'
    },
    measurements: {
      required: true,
      type: Array
    },
    name: {
      required: true,
      type: String
    },
    options: {
      type: Object,
      required: false,
      default: () => ({})
    }
  },
  computed: {
    chartOptions() {
      return objectAssignDeep({
        chart: {
          animations: {
            speed: 500,
            animateGradually: {
              enabled: false
            },
            dynamicAnimation: {
              speed: 200
            }
          },
          events: {
            zoomed: () => {
              this.$emit('zoomed-in')
            }
          },
          stacked: true,
          toolbar: {
            show: false
          },
          zoom: {
            enabled: true
          }
        },
        dataLabels: {
          enabled: false
        },
        fill: {
          colors: ['green']
        },
        grid: {
          row: {
            colors: ['#f3f3f3', 'transparent'], // takes an array which will be repeated on columns
            opacity: 0.5
          }
        },
        stroke: {
          curve: 'straight',
          width: 2
        },
        tooltip: {
          enabled: false
        },
        xaxis: {
          hideOverlappingLabels: true,
          labels: {
            datetimeFormatter: {
              year: 'yyyy',
              month: "MMM 'yy",
              day: 'dd MMM',
              hour: 'HH:mm'
            },
            datetimeUTC: false
          },
          type: 'datetime'
        }
      }, this.options)
    },
    series() {
      return [
        {
          name: this.name,
          data: this.measurements.map(m => ({
            // Adding timezone offset tells javascript these measurement timestamps are in UTC
            x: new Date(m.created_at + '+00:00'),
            y: m.value
          }))
        }
      ]
    }
  },
  methods: {
    resetZoom() {
      this.$refs.chart.resetSeries(false)
    }
  }
}
</script>

<style scoped>
.chart {
  padding-top: -14px;
}
</style>
