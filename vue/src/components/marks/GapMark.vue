<template>
  <input
    v-model="value"
    :class="{
      correct: checked && correct,
      incorrect: checked && !correct,
    }"
  />
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
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
      return payload.achievedPoints <= payload.totalPoints;
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
.correct {
  background-color: rgba(#00ff00, 0.5);
}
.incorrect {
  background-color: rgba(#ff0000, 0.5);
}
</style>
