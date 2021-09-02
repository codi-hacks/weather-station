<template>
  <div class="header">
    <slot />
  </div>
</template>

<style scoped>
.header {
  position: absolute;
  left: 12%;
  font-size: 1.14em;
  font-weight: bold;
  z-index: 1;
  text-shadow: 2px 2px var(--v-text-inverse-base);
}
</style>>
