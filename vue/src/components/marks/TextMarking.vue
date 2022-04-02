<template>
  <component
    :is="markType"
    :mark="marks[0]"
    :markExport="marksExport[0]"
    :checkTrigger="checkTrigger"
    @grantPoints="forwardGrantPoints"
  >
    <text-marking
      v-if="hasMoreMarks"
      :marks="marks.slice(1)"
      :marksExport="marksExport.slice(1)"
      :checkTrigger="checkTrigger"
      @grantPoints="forwardGrantPoints"
    >
      <slot></slot>
    </text-marking>
    <slot v-else></slot>
  </component>
</template>

<script setup lang="ts">
import { computed, toRefs } from "vue";

import { Mark } from "../../model/SheetDisplayMark";

import BoldMark from "./BoldMark.vue";
import GapMark from "./GapMark.vue";
import ItalicMark from "./ItalicMark.vue";
import LatexMark from "./LatexMark.vue";
import StrikeMark from "./StrikeMark.vue";

const propsDef = defineProps({
  marks: {
    type: Array as () => Mark[],
    required: true,
    validator(mark: Mark[]) {
      return mark.length > 0;
    },
  },
  marksExport: {
    type: Array as () => Mark[],
    required: true,
    validator(mark: Mark[]) {
      return mark.length > 0;
    },
  },
  checkTrigger: {
    type: Boolean,
    required: true,
  },
});
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

const hasMoreMarks = computed(() => props.marks.value.length > 1);

const componentMap: { [key: string]: unknown } = {
  bold: BoldMark,
  gap: GapMark,
  italic: ItalicMark,
  latex: LatexMark,
  strike: StrikeMark,
  fallback: "span",
};
const markType = computed(
  () => componentMap[props.marks.value[0].type?.toLowerCase() ?? "fallback"]
);
</script>

<style></style>
