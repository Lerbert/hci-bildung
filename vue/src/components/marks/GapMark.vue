<template>
  <div class="is-inline-block">
    <div class="field has-addons">
      <div class="control has-icons-right">
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
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            width="20px"
            height="20px"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M5 13l4 4L19 7"
            />
          </svg>
        </span>
        <span v-if="wrong" class="icon is-small is-right">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            width="20px"
            height="20px"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </span>
      </div>
    </div>
  </div>
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
}
.icon {
  width: 28px !important;
  height: 28px !important;
}
</style>
