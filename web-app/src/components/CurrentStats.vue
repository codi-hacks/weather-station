<template>
  <ul class="current-stats">
    <li></li>
    <li>
      <h3>Current</h3>
      <h2><slot name="realtime" /></h2>
    </li>
    <li>
      <h3>Average</h3>
      <h2><slot name="average" /></h2>
    </li>
    <li></li>
  </ul>
</template>

<style scoped>
.current-stats {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  height: 100%;
}

.current-stats li {
  align-content: center;
  display: flex;
  flex-basis: 100%;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
</style>
