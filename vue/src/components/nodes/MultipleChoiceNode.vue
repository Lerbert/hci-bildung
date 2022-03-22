<template>
  <ul
    data-type="multipleChoice"
    :class="{
      correct: checked && correct,
      incorrect: checked && !correct,
    }"
  >
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
</template>

<script>
import { defineAsyncComponent, defineComponent } from "vue";

import MultipleChoiceAnswerNode from "./MultipleChoiceAnswerNode.vue";

export default defineComponent({
  components: {
    MultipleChoiceAnswerNode,
    SheetNode: defineAsyncComponent(() => import("./SheetNode.vue")),
  },

  props: {
    checkTrigger: {
      type: Boolean,
      required: true,
    },
    tiptapNode: {
      type: Object,
      required: true,
      validator(value) {
        return value.content.every((c) => c.type === "multipleChoiceAnswer");
      },
    },
  },

  emits: {
    grantPoints(payload) {
      return payload.achievedPoints <= payload.totalPoints;
    },
  },

  data() {
    return {
      checkAnswersTrigger: false,
      checked: false,
      correct: false,
      correctAnswers: 0,
      totalAnswers: 0,
      achievedPoints: 0,
      totalPoints: 1,
    };
  },

  computed: {
    expectedAnswers() {
      return this.tiptapNode.content.length;
    },
  },

  mounted() {
    this.$emit("grantPoints", {
      achievedPoints: this.achievedPoints,
      totalPoints: this.totalPoints,
    });
  },

  beforeUnmount() {
    this.$emit("grantPoints", {
      achievedPoints: -this.achievedPoints,
      totalPoints: -this.totalPoints,
    });
  },

  methods: {
    resetCorrectAnswerCount() {
      this.correctAnswers = 0;
      this.totalAnswers = 0;
    },
    countAnswerCorrect(event) {
      if (event.correct) {
        this.correctAnswers++;
      }
      this.totalAnswers++;
      // If we got all responses check this element
      if (this.totalAnswers === this.expectedAnswers) {
        this.check();
      }
    },
    check() {
      this.checked = true;
      this.correct = this.correctAnswers === this.totalAnswers;
      this.achievedPoints = this.correct ? this.totalPoints : 0;
      this.$emit("grantPoints", {
        achievedPoints: this.achievedPoints,
        totalPoints: this.totalPoints,
      });
    },
  },

  watch: {
    checkTrigger() {
      // First reset answers ...
      this.resetCorrectAnswerCount();
      // ... and trigger answer check only once we are done with this
      this.checkAnswersTrigger = !this.checkAnswersTrigger;
    },
  },
});
</script>

<style lang="scss" scoped>
ul[data-type="multipleChoice"] {
  list-style: none;
  padding: 0;
}
</style>
