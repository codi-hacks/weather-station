<template>
  <div class="bookmark-button" :class="{ 'edit-mode': editMode }">
    <v-btn x-small @click="toggleBookmark()">
      <v-icon v-if="isDashboard" small>mdi-trash-can-outline</v-icon>
      <v-icon v-else-if="bookmarked">mdi-bookmark-check-outline</v-icon>
      <v-icon v-else>mdi-bookmark-plus</v-icon>
    </v-btn>
  </div>
</template>

<script>
import { mapMutations } from 'vuex'

export default {
  props: {
    editMode: {
      required: true,
      type: Boolean
    },
    isDashboard: {
      required: true,
      type: Boolean
    },
    mode: {
      required: true,
      type: String
    },
    sensorId: {
      required: true,
      type: String
    },
    timeAgo: {
      required: true,
      type: Number
    }
  },
  computed: {
    bookmarked() {
      return Boolean(
        this.$store.state.dashboard.find(sensor => sensor.id === this.sensorId)
      )
    }
  },
  methods: {
    toggleBookmark() {
      if (this.bookmarked) {
        this.$store.commit('removeBookmark', this.sensorId)
      } else {
        this.$store.commit('addBookmark', {
          id: this.sensorId,
          mode: this.mode,
          timeAgo: this.timeAgo
        })
      }
    },
    ...mapMutations([
      'addBookmark',
      'removeBookmark'
    ])
  }
}
</script>

<style scoped>
.bookmark-button {
  bottom: 0;
  display: none;
  padding-bottom: 4px;
  padding-right: 4px;
  position: absolute;
  right: 0;
  z-index: 1;
}

@media screen and (max-width: 1000px) {
  .bookmark-button.edit-mode {
    display: block;
  }
}
</style>
