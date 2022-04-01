<template>
  <component
    :is="markType"
    :mark="marks[0]"
    :checkTrigger="checkTrigger"
    @grantPoints="(event) => $emit('grantPoints', event)"
  >
    <text-marking
      v-if="hasMoreMarks"
      :marks="marks.slice(1)"
      :checkTrigger="checkTrigger"
      @grantPoints="(event) => $emit('grantPoints', event)"
    >
      <slot />
    </text-marking>
    <slot v-else />
  </component>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import BoldMark from "./BoldMark.vue";
import GapMark from "./GapMark.vue";
import ItalicMark from "./ItalicMark.vue";
import LatexMark from "./LatexMark.vue";
import StrikeMark from "./StrikeMark.vue";

export default defineComponent({
  // For recursion
  name: "TextMarking",

  components: {
    BoldMark,
    GapMark,
    ItalicMark,
    LatexMark,
    StrikeMark,
  },

  props: {
    marks: {
      type: Object,
      required: true,
      validator(value: any) {
        return value.length > 0;
      },
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
    hasMoreMarks(): boolean {
      return this.marks.length > 1;
    },
    markType(): string {
      if (this.marks.length === 0) {
        return "";
      } else {
        return this.marks[0].type.toLowerCase() + "-mark";
      }
    },
  },
});
</script>

<style></style>
