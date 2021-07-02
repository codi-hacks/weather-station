<template>
    <ul class="current-container">
      <li></li>
      <li>
      <div class="header"> Current Temperature: </div>
      <div class="value"> <slot name="realtime"/> </div>
      </li>
      <li>
      <div class="header"> Average Temperature: </div>
      <div class="value"> <slot name="average">Err</slot> </div>
      </li>
      <li></li>
    </ul>
</template>

<style scoped>
.current-container {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  height: 100%;
}

.current-container li {
  color: #1976d2;
  align-content: center;
  display: flex;
  flex-basis: 100%;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.header {
  font-size: 2.5em;
}

.value {
  font-size: 3.5em;
}
</style>
