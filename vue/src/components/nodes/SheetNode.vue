<template>
  <text-marking
    v-if="sheet.marks.length > 0"
    :marks="sheet.marks"
    :checkTrigger="checkTrigger"
    @grantPoints="(event) => $emit('grantPoints', event)"
  >
    <component
      :is="nodeType"
      :sheet="sheet"
      :checkTrigger="checkTrigger"
      @grantPoints="(event) => $emit('grantPoints', event)"
    >
      <sheet-node
        v-for="(c, index) in sheet.content"
        :key="index"
        :sheet="c"
        :checkTrigger="checkTrigger"
        @grantPoints="(event) => $emit('grantPoints', event)"
      ></sheet-node>
    </component>
  </text-marking>
  <component
    v-else
    :is="nodeType"
    :sheet="sheet"
    :checkTrigger="checkTrigger"
    @grantPoints="(event) => $emit('grantPoints', event)"
  >
    <sheet-node
      v-for="(c, index) in sheet.content"
      :key="index"
      :sheet="c"
      :checkTrigger="checkTrigger"
      @grantPoints="(event) => $emit('grantPoints', event)"
    ></sheet-node>
  </component>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import TextMarking from "../marks/TextMarking.vue";

import AudioNode from "./AudioNode.vue";
import BulletListNode from "./BulletListNode.vue";
import CodeblockNode from "./CodeblockNode.vue";
import DocNode from "./DocNode.vue";
import HardbreakNode from "./HardbreakNode.vue";
import HeadingNode from "./HeadingNode.vue";
import ListItemNode from "./ListItemNode.vue";
import MultipleChoiceAnswerNode from "./MultipleChoiceAnswerNode.vue";
import MultipleChoiceNode from "./MultipleChoiceNode.vue";
import OrderedListNode from "./OrderedListNode.vue";
import ParagraphNode from "./ParagraphNode.vue";
import TextNode from "./TextNode.vue";

export default defineComponent({
  // For recursion
  name: "SheetNode",

  components: {
    TextMarking,
    AudioNode,
    BulletlistNode: BulletListNode,
    CodeblockNode,
    DocNode,
    HardbreakNode,
    HeadingNode,
    ListitemNode: ListItemNode,
    MultiplechoiceanswerNode: MultipleChoiceAnswerNode,
    MultiplechoiceNode: MultipleChoiceNode,
    OrderedlistNode: OrderedListNode,
    ParagraphNode,
    TextNode,
  },

  props: {
    sheet: {
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
      return (
        payload.achievedPoints <= payload.totalPoints ||
        (payload.totalPoints < 0 && payload.achievedPoints <= 0)
      );
    },
  },

  computed: {
    nodeType(): string {
      return this.sheet.type.toLowerCase() + "-node";
    },
  },
});
</script>

<style></style>
