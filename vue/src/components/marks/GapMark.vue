<template>
  <div class="is-inline-block">
    <div class="field has-addons">
      <div class="control" :class="{ 'has-icons-right': right || wrong }">
        <input
          v-model="value"
          class="input"
          :class="{
            correct: right,
            incorrect: wrong,
          }"
          :style="{ width: `${width}rem` }"
        />
        <span v-if="right" class="icon is-small is-right">
          <check-symbol></check-symbol>
        </span>
        <span v-if="wrong" class="icon is-small is-right">
          <cross-symbol></cross-symbol>
        </span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import CheckSymbol from "../feedback_symbols/CheckSymbol.vue";
import CrossSymbol from "../feedback_symbols/CrossSymbol.vue";

export default defineComponent({
  components: {
    CheckSymbol,
    CrossSymbol,
  },

  props: {
    checkTrigger: {
      type: Boolean,
      required: true,
    },
    tiptapNode: {
      type: Object,
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

  data() {
    return {
      value: "",
      checked: false,
      correct: false,
      achievedPoints: 0,
      totalPoints: 1,
    };
  },

  computed: {
    solution(): string {
      return this.tiptapNode.text;
    },
    width(): number {
      // Lower resolution to multiples of 5 to not reveal the exact solution length
      return Math.ceil(this.solution.length / 5) * 5;
    },
    right(): boolean {
      return this.checked && this.correct;
    },
    wrong(): boolean {
      return this.checked && !this.correct;
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
    check() {
      this.checked = true;
      this.correct = this.value === this.solution;
      this.achievedPoints = this.correct ? this.totalPoints : 0;
      this.$emit("grantPoints", {
        achievedPoints: this.achievedPoints,
        totalPoints: this.totalPoints,
      });
    },
  },

  watch: {
    checkTrigger() {
      this.check();
    },
  },
});
</script>

<style lang="scss" scoped>
.input {
  height: auto;
  padding: 1px 2px;
  margin-bottom: 3px;
  border-radius: 2px;
  vertical-align: baseline;

  &.correct,
  &.incorrect {
    padding-right: 28px;
  }
}
.icon {
  width: 28px !important;
  height: 28px !important;
  color: unset !important;
}
</style>
