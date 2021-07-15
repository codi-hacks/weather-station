<template>
  <ul class="container">
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
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  height: 100%;
}

.container li {
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

@media screen and (max-width: 640px) {
  .container li {
    flex-basis: 50%;
    flex-direction: column wrap;
    flex-flow: column;
    flex-grow: 1;
    justify-content: center;
    text-align: center;
  }

  .container li:first-child {
    display: none;
  }

  .container li:last-child {
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
