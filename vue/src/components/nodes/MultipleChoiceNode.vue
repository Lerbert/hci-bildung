<template>
  <div
    :class="{
      correct: right,
      incorrect: wrong,
    }"
  >
    <ul data-type="multipleChoice">
      <multiple-choice-answer-node
        v-for="(answer, i) in sheet.content"
        :key="i"
        :sheet="answer"
        :sheetExport="sheetExport.content[i]"
        :checkTrigger="checkAnswersTrigger"
        @grantPoints="forwardGrantPoints"
        @answerCorrect="countAnswerCorrect"
      >
        <sheet-node
          v-for="(c, j) in answer.content"
          :key="j"
          :sheet="c"
          :sheetExport="sheetExport.content[i].content[j]"
          :checkTrigger="checkTrigger"
          @grantPoints="forwardGrantPoints"
        ></sheet-node>
      </multiple-choice-answer-node>
    </ul>
    <span class="icon">
      <check-symbol v-if="right"></check-symbol>
      <cross-symbol v-if="wrong"></cross-symbol>
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, toRefs, watch } from "vue";

import { useCheckable, withCheckableEmit } from "../../composables/Checkable";
import { MultipleChoice } from "../../model/SheetDisplayNode";

import CheckSymbol from "../feedback_symbols/CheckSymbol.vue";
import CrossSymbol from "../feedback_symbols/CrossSymbol.vue";
import MultipleChoiceAnswerNode from "./MultipleChoiceAnswerNode.vue";
import SheetNode from "./SheetNode.vue";

const propsDef = defineProps<{
  sheet: MultipleChoice;
  sheetExport: MultipleChoice;
  checkTrigger: boolean;
}>();
const props = toRefs(propsDef);

const emit = defineEmits({
  ...withCheckableEmit(),
});

function forwardGrantPoints(event: {
  achievedPoints: number;
  totalPoints: number;
}) {
  return emit("grantPoints", event);
}

const checkAnswersTrigger = ref(false);
const correctAnswers = ref(0);
const totalAnswers = ref(0);
const expectedAnswers = computed(() => props.sheet.value.content.length);
function resetCorrectAnswerCount() {
  correctAnswers.value = 0;
  totalAnswers.value = 0;
}
function countAnswerCorrect(correct: boolean) {
  if (correct) {
    correctAnswers.value++;
  }
  totalAnswers.value++;
  // If we got all responses check this element
  if (totalAnswers.value === expectedAnswers.value) {
    allCheckedTrigger.value = !allCheckedTrigger.value;
  }
}
watch(props.checkTrigger, () => {
  // First reset answers ...
  resetCorrectAnswerCount();
  // ... and trigger answer check only once we are done with this
  checkAnswersTrigger.value = !checkAnswersTrigger.value;
});

const totalPoints = 1;
const allCheckedTrigger = ref(false);
function check() {
  return correctAnswers.value === totalAnswers.value ? totalPoints : 0;
}
const { right, wrong } = useCheckable(
  allCheckedTrigger,
  emit,
  check,
  totalPoints
);
</script>

<style lang="scss" scoped>
div {
  display: flex;
  align-items: center;
  justify-content: space-between;

  &.correct,
  &.incorrect {
    border-width: 1px;
    border-style: solid;
  }

  ul[data-type="multipleChoice"] {
    list-style: none;
    padding: 0;
    min-width: 0%;
  }
}
</style>
