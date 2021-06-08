<template>
  <slot v-if="empty"/>
  <component v-else :is="markType">
    <marking :marks="marks.slice(1)"><slot/></marking>
  </component>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import BoldMark from './BoldMark.vue';
import ItalicMark from './ItalicMark.vue';
import StrikeMark from './StrikeMark.vue';

export default defineComponent({
  // For recursion
  name: 'marking',

  components: {
    BoldMark,
    ItalicMark,
    StrikeMark,
  },

  props: {
    marks: {
      type: Array as () => Array<any>,
      required: true,
    }
  },

  computed: {
    empty(): boolean {
      return this.marks.length === 0
    },
    markType(): string {
      if (this.empty) {
        return '';
      } else {
        return this.marks[0].type.toLowerCase() + '-mark';
      }
    }
  }
});
</script>

<style>

</style>