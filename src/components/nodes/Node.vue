<template>
  <marking v-if="tiptapNode.marks" :tiptapNode="tiptapNode" :checkTrigger="checkTrigger" @grantPoints="(event) => $emit('grantPoints', event)">
    <component :is="nodeType" :tiptapNode="tiptapNode" @grantPoints="(event) => $emit('grantPoints', event)">
      <node v-for="(c, index) in tiptapNode.content" :key="index" :tiptapNode="c" :checkTrigger="checkTrigger" @grantPoints="(event) => $emit('grantPoints', event)"></node>
    </component>
  </marking>
  <component v-else :is="nodeType" :tiptapNode="tiptapNode" @grantPoints="(event) => $emit('grantPoints', event)">
      <node v-for="(c, index) in tiptapNode.content" :key="index" :tiptapNode="c" :checkTrigger="checkTrigger" @grantPoints="(event) => $emit('grantPoints', event)"></node>
  </component>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import Marking from '../marks/Marking.vue';

import AudioNode from './AudioNode.vue';
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
    Marking,
    AudioNode,
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
    },
    checkTrigger: {
      type: Boolean,
      required: true
    },
  },

  emits: {
    grantPoints(payload: { achievedPoints: number; totalPoints: number }) {
      return payload.achievedPoints <= payload.totalPoints;
    },
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