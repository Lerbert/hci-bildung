<template>
  <div class="points_container">
    <svg
      v-if="achievedPoints === totalPoints"
      color="#48c774"
      xmlns="http://www.w3.org/2000/svg"
      class="h-6 w-6"
      fill="none"
      viewBox="0 0 24 24"
      height="18px"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M5 13l4 4L19 7"
      />
    </svg>
    <svg
      v-if="achievedPoints < totalPoints"
      color="#ffd026"
      xmlns="http://www.w3.org/2000/svg"
      class="h-6 w-6"
      fill="none"
      viewBox="0 0 24 24"
      height="18px"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
      />
    </svg>
    <span
      >{{ achievedPoints }} von {{ totalPoints }} Punkt<span
        v-if="totalPoints !== 1"
        >en</span
      >
      erreicht</span
    >
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    achievedPoints: {
      type: Number,
      required: true,
    },
    totalPoints: {
      type: Number,
      required: true,
    },
  },
});
</script>

<style lang="scss" scoped>
.points_container {
  display: flex;
  align-items: space-between;
  flex: 0 0 auto;
  flex-wrap: wrap;
}
</style>
