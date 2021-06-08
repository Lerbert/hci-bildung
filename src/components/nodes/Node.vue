<template>
  <component :is="nodeType" :tiptapNode="tiptapNode">
      <node v-for="(c, index) in tiptapNode.content" :key="index" :tiptapNode="c"></node>
  </component>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import BulletlistNode from './BulletlistNode.vue';
import CodeblockNode from './CodeblockNode.vue';
import DocNode from './DocNode.vue';
import HardbreakNode from './HardbreakNode.vue';
import HeadingNode from './HeadingNode.vue';
import ListitemNode from './ListitemNode.vue';
import OrderedlistNode from './OrderedlistNode.vue';
import ParagraphNode from './ParagraphNode.vue';
import TextNode from './TextNode.vue';

export default defineComponent({
  // For recursion
  name: 'node',

  components: {
    BulletlistNode,
    CodeblockNode,
    DocNode,
    HardbreakNode,
    HeadingNode,
    ListitemNode,
    OrderedlistNode,
    ParagraphNode,
    TextNode,
  },

  props: {
    tiptapNode: {
      type: Object,
      required: true,
    }
  },

  computed: {
    nodeType(): string {
      return this.tiptapNode.type.toLowerCase() + '-node';
    }
  }
});
</script>

<style>

</style>