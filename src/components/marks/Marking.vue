<template>
  <slot v-if="empty"/>
  <component v-else :is="markType" :tiptapNode="tiptapNode" :checkTrigger="checkTrigger">
    <marking :tiptapNode="{...tiptapNode, marks: marks.slice(1)}" :checkTrigger="checkTrigger"><slot/></marking>
  </component>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import BoldMark from './BoldMark.vue';
import GapMark from './GapMark.vue';
import ItalicMark from './ItalicMark.vue';
import StrikeMark from './StrikeMark.vue';

export default defineComponent({
  // For recursion
  name: 'marking',

  components: {
    BoldMark,
    GapMark,
    ItalicMark,
    StrikeMark,
  },

  props: {
    tiptapNode: {
      type: Object,
      required: true,
    },
    checkTrigger: {
      type: Boolean,
      required: true
    },
  },

  computed: {
    marks(): any[] {
      return this.tiptapNode.marks;
    },
    empty(): boolean {
      return this.marks.length == 0
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