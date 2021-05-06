<template>
  <VueApexCharts width="500" type="line" :options="chartOptions" :series="series"/>
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
    /* eslint-disable */
    const sensors = {
      "id": "819445e4-767c-41b1-8699-1a4a98333213",
      "alias": "air_temp",
      "label": "Air temperature",
      "measurements": [
        {
          "id": "04607369-4d59-4423-80ab-62464c0f1c0d",
          "value": "22",
          "created_at": "2021-05-06T16:07:06.754421"
        },
        {
          "id": "04607369-4d59-4423-80ab-62464c0f1c0d",
          "value": "26",
          "created_at": "2021-05-06T16:07:06.754421"
        },
        {
          "id": "04607369-4d59-4423-80ab-62464c0f1c0d",
          "value": "27",
          "created_at": "2021-05-06T16:07:06.754421"
        }
      ],
      "type": {
        "id": "6a3afc2e-f38e-4044-972d-49388f51053a",
        "label": "celsius",
        "description": "celsius"
      },
      "station": {
        "id": "eb54ffd7-34fb-4599-a5df-efdd2b8bf46c",
        "label": "Hortonia"
      },
      "created_at": "2021-04-27T21:01:10.136642",
      "updated_at": "2021-04-27T21:01:10.136642"
    }

    const measurements = sensors.measurements.map(m => m.value)


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
          name: 'Temperature',
          data: measurements
        }
      ]
    }
  }
}
</script>
