<template>
  <input v-model="value" v-bind:class="!checked ? '' : correct ? 'correct' : 'incorrect'">
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    checkTrigger: {
      type: Boolean,
      required: true,
    },
    tiptapNode: {
      type: Object,
      required: true,
    }
  },

  data() {
    return {
      value: '',
      checked: false,
      correct: false,
    }
  },

  computed: {
    solution(): string {
      return this.tiptapNode.text;
    }
  },

  methods: {
    check() {
      this.checked = true;
      this.correct = this.value === this.solution;
    }
  },

  watch: {
    checkTrigger() {
      this.check();
    }
  }
});
</script>

<style lang="scss" scoped>
  .correct {
    background-color: rgba(#00ff00, 0.5);
  }
  .incorrect {
    background-color: rgba(#ff0000, 0.5);
  }
</style>