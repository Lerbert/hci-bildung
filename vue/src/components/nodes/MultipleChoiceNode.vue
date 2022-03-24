<template>
  <div
    :class="{
      correct: right,
      incorrect: wrong,
    }"
  >
    <ul data-type="multipleChoice">
      <multiple-choice-answer-node
        v-for="(c, index) in tiptapNode.content"
        :key="index"
        :tiptapNode="c"
        :checkTrigger="checkAnswersTrigger"
        @grantPoints="(event) => $emit('grantPoints', event)"
        @answerCorrect="countAnswerCorrect"
      >
        <sheet-node
          v-for="(c, index) in c.content"
          :key="index"
          :tiptapNode="c"
          :checkTrigger="checkTrigger"
          @grantPoints="(event) => $emit('grantPoints', event)"
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
import { computed, onBeforeUnmount, onMounted, ref, toRefs, watch } from "vue";

import CheckSymbol from "../feedback_symbols/CheckSymbol.vue";
import CrossSymbol from "../feedback_symbols/CrossSymbol.vue";
import MultipleChoiceAnswerNode from "./MultipleChoiceAnswerNode.vue";
import SheetNode from "./SheetNode.vue";

const propsDef = defineProps({
  checkTrigger: {
    type: Boolean,
    required: true,
  },
  tiptapNode: {
    type: Object,
    required: true,
    validator(value: Record<string, any>) {
      return value.content.every((c) => c.type === "multipleChoiceAnswer");
    },
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

const checkAnswersTrigger = ref(false);
const checked = ref(false);
const correct = ref(false);
const correctAnswers = ref(0);
const totalAnswers = ref(0);
const achievedPoints = ref(0);
const totalPoints = ref(1);

const expectedAnswers = computed(() => props.tiptapNode.value.content.length);
const right = computed(() => checked.value && correct.value);
const wrong = computed(() => checked.value && !correct.value);

onMounted(() =>
  emit("grantPoints", {
    achievedPoints: achievedPoints.value,
    totalPoints: totalPoints.value,
  })
);

onBeforeUnmount(() =>
  emit("grantPoints", {
    achievedPoints: -achievedPoints.value,
    totalPoints: -totalPoints.value,
  })
);

function resetCorrectAnswerCount() {
  correctAnswers.value = 0;
  totalAnswers.value = 0;
}

function countAnswerCorrect(event) {
  if (event.correct) {
    correctAnswers.value++;
  }
  totalAnswers.value++;
  // If we got all responses check this element
  if (totalAnswers.value === expectedAnswers.value) {
    check();
  }
}

function check() {
  checked.value = true;
  correct.value = correctAnswers.value === totalAnswers.value;
  achievedPoints.value = correct.value ? totalPoints.value : 0;
  emit("grantPoints", {
    achievedPoints: achievedPoints.value,
    totalPoints: totalPoints.value,
  });
}

watch(props.checkTrigger, () => {
  // First reset answers ...
  resetCorrectAnswerCount();
  // ... and trigger answer check only once we are done with this
  checkAnswersTrigger.value = !checkAnswersTrigger.value;
});
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
