<template>
  <div class="header" :class="{ 'edit-mode': editMode }">
    <slot />
  </div>
</template>

<script>
export default {
  props: {
    editMode: {
      required: true,
      type: Boolean
    }
  }
}
</script>

<style scoped>
/* Offset to account for buttons when in edit mode */
.edit-mode {
  left: 12%;
}

.header {
  position: absolute;
  font-size: 1.14em;
  font-weight: bold;
  z-index: 1;
  text-shadow: 2px 2px var(--v-text-inverse-base);
}

/* Buttons are always present on larger screen widths */
@media screen and not (max-width: 1000px) {
  .header {
    left: 12%;
  }
}
</style>
