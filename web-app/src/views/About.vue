<template>
  <v-container>
    <div class="about">
      <h1>This is an about page</h1>
      <div>
        <Graph/>
      </div>
    </div>
  </v-container>
</template>

<script>
import Graph from '../components/Graph'
export default {
  components: {
    Graph
  }
}
</script>
