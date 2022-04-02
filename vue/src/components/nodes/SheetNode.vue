<template>
  <text-marking
    v-if="sheet.marks.length > 0"
    :marks="sheet.marks"
    :marksExport="sheetExport.marks"
    :checkTrigger="checkTrigger"
    @grantPoints="forwardGrantPoints"
  >
    <component
      :is="nodeType"
      :sheet="sheet"
      :sheetExport="sheetExport"
      :checkTrigger="checkTrigger"
      @grantPoints="forwardGrantPoints"
    >
      <sheet-node
        v-for="(c, index) in sheet.content"
        :key="index"
        :sheet="c"
        :sheetExport="sheetExport.content[index]"
        :checkTrigger="checkTrigger"
        @grantPoints="forwardGrantPoints"
      ></sheet-node>
    </component>
  </text-marking>
  <component
    v-else
    :is="nodeType"
    :sheet="sheet"
    :sheetExport="sheetExport"
    :checkTrigger="checkTrigger"
    @grantPoints="forwardGrantPoints"
  >
    <sheet-node
      v-for="(c, index) in sheet.content"
      :key="index"
      :sheet="c"
      :sheetExport="sheetExport.content[index]"
      :checkTrigger="checkTrigger"
      @grantPoints="forwardGrantPoints"
    ></sheet-node>
  </component>
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent, toRefs } from "vue";

import { Node } from "../../model/SheetDisplayNode";

import AudioNode from "./AudioNode.vue";
import BulletListNode from "./BulletListNode.vue";
import CodeblockNode from "./CodeblockNode.vue";
import DocNode from "./DocNode.vue";
import HardbreakNode from "./HardbreakNode.vue";
import HeadingNode from "./HeadingNode.vue";
import ListItemNode from "./ListItemNode.vue";
import MultipleChoiceAnswerNode from "./MultipleChoiceAnswerNode.vue";
import OrderedListNode from "./OrderedListNode.vue";
import ParagraphNode from "./ParagraphNode.vue";
import TextMarking from "../marks/TextMarking.vue";
import TextNode from "./TextNode.vue";

const propsDef = defineProps<{
  sheet: Node;
  sheetExport: Node;
  checkTrigger: boolean;
}>();
const props = toRefs(propsDef);

const emit = defineEmits({
  grantPoints(payload: { achievedPoints: number; totalPoints: number }) {
    return (
      payload.achievedPoints <= payload.totalPoints ||
      (payload.totalPoints < 0 && payload.achievedPoints <= 0)
    );
  },
});

function forwardGrantPoints(event: {
  achievedPoints: number;
  totalPoints: number;
}) {
  return emit("grantPoints", event);
}

const componentMap: { [key: string]: unknown } = {
  audio: AudioNode,
  bulletList: BulletListNode,
  codeBlock: CodeblockNode,
  doc: DocNode,
  hardbreak: HardbreakNode,
  heading: HeadingNode,
  listItem: ListItemNode,
  multipleChoiceAnswer: MultipleChoiceAnswerNode,
  multipleChoice: defineAsyncComponent(
    () => import("./MultipleChoiceNode.vue")
  ),
  orderedList: OrderedListNode,
  paragraph: ParagraphNode,
  textMarking: TextMarking,
  text: TextNode,
  fallback: "div",
};
const nodeType = computed(
  () => componentMap[props.sheet.value?.type ?? "fallback"]
);
</script>

<style></style>
