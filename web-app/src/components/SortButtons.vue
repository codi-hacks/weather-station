<template>
  <div class="sort-buttons" :class="{ 'edit-mode': editMode }">
    <v-btn x-small @click="moveCard('left')">
      <v-icon class="up-arrow">mdi-arrow-up-thick</v-icon>
      <v-icon class="left-arrow">mdi-arrow-left-thick</v-icon>
    </v-btn>
    <v-btn x-small @click="moveCard('right')">
      <v-icon class="down-arrow">mdi-arrow-down-thick</v-icon>
      <v-icon class="right-arrow">mdi-arrow-right-thick</v-icon>
    </v-btn>
  </div>
</template>

<script>
export default {
  props: {
    editMode: {
      required: true,
      type: Boolean
    },
    sensorId: {
      required: true,
      type: String
    }
  },
  methods: {
    moveCard(direction) {
      this.$store.commit('moveSensorCard', {
        sensorId: this.sensorId,
        direction
      })
    }
  }
}
</script>

<style scoped>
.sort-buttons {
  bottom: 0;
  display: none;
  left: 0;
  padding-bottom: 4px;
  padding-left: 4px;
  position: absolute;
  z-index: 1;
}

.sort-buttons.edit-mode {
  display: block;
}

.up-arrow {
  display: none;
}
.down-arrow {
  display: none;
}
.left-arrow {
  display: block;
}
.right-arrow {
  display: block;
}

/* Display up/down arrows instead on small displays
   since we're now in a vertical layout. */
@media screen and (max-width: 640px) {
  .up-arrow {
    display: block;
  }
  .down-arrow {
    display: block;
  }
  .left-arrow {
    display: none;
  }
  .right-arrow {
    display: none;
  }
}
</style>
