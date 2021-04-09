<template>
  <VueApexCharts width="500" type="bar" :options="chartOptions" :series="series"/>
</template>

<script>
import VueApexCharts from 'vue-apexcharts'
export default {
  components: {
    VueApexCharts
  },
  props: {
    name: {
      required: true
    }
  },
  data: function() {
    return {
      chartOptions: {
        chart: {
          id: 'vuechart-example'
        },
        xaxis: {
          categories: ['June', 'July', 'August']
        }
      },
      series: [
        {
          name: 'Average High',
          data: [80, 91, 86]
        },
        {
          name: 'Average Low',
          data: [56, 70, 66]
        }
      ]
    }
  }
}
</script>
