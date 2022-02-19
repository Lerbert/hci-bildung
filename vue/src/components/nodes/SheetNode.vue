<template>
  <text-marking
    v-if="tiptapNode.marks"
    :tiptapNode="tiptapNode"
    :checkTrigger="checkTrigger"
    @grantPoints="(event) => $emit('grantPoints', event)"
  >
    <component
      :is="nodeType"
      :tiptapNode="tiptapNode"
      @grantPoints="(event) => $emit('grantPoints', event)"
    >
      <sheet-node
        v-for="(c, index) in tiptapNode.content"
        :key="index"
        :tiptapNode="c"
        :checkTrigger="checkTrigger"
        @grantPoints="(event) => $emit('grantPoints', event)"
      ></sheet-node>
    </component>
  </text-marking>
  <component
    v-else
    :is="nodeType"
    :tiptapNode="tiptapNode"
    @grantPoints="(event) => $emit('grantPoints', event)"
  >
    <sheet-node
      v-for="(c, index) in tiptapNode.content"
      :key="index"
      :tiptapNode="c"
      :checkTrigger="checkTrigger"
      @grantPoints="(event) => $emit('grantPoints', event)"
    ></sheet-node>
  </component>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import TextMarking from "../marks/TextMarking.vue";

import AudioNode from "./AudioNode.vue";
import BulletlistNode from "./BulletlistNode.vue";
import CodeblockNode from "./CodeblockNode.vue";
import DocNode from "./DocNode.vue";
import HardbreakNode from "./HardbreakNode.vue";
import HeadingNode from "./HeadingNode.vue";
import ListitemNode from "./ListitemNode.vue";
import OrderedlistNode from "./OrderedlistNode.vue";
import ParagraphNode from "./ParagraphNode.vue";
import TextNode from "./TextNode.vue";

export default defineComponent({
  // For recursion
  name: "SheetNode",

  components: {
    TextMarking,
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
      required: true,
    },
  },

  emits: {
    grantPoints(payload: { achievedPoints: number; totalPoints: number }) {
      return payload.achievedPoints <= payload.totalPoints;
    },
  },

  computed: {
    nodeType(): string {
      return this.tiptapNode.type.toLowerCase() + "-node";
    },
  },
});
</script>

<style></style>
