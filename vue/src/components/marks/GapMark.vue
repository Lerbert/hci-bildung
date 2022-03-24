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
        <span class="icon is-small is-right">
          <check-symbol v-if="right"></check-symbol>
          <cross-symbol v-if="wrong"></cross-symbol>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, toRefs, watch } from "vue";

import CheckSymbol from "../feedback_symbols/CheckSymbol.vue";
import CrossSymbol from "../feedback_symbols/CrossSymbol.vue";

const propsDef = defineProps<{
  checkTrigger: boolean;
  tiptapNode: Record<string, any>;
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

const value = ref("");
const checked = ref(false);
const correct = ref(false);
const achievedPoints = ref(0);
const totalPoints = ref(1);

const solution = computed(() => props.tiptapNode.value.text);
// Lower resolution to multiples of 5 to not reveal the exact solution length
const width = computed(() => Math.ceil(solution.value.length / 5) * 5);
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

function check() {
  checked.value = true;
  correct.value = value.value === solution.value;
  achievedPoints.value = correct.value ? totalPoints.value : 0;
  emit("grantPoints", {
    achievedPoints: achievedPoints.value,
    totalPoints: totalPoints.value,
  });
}

watch(props.checkTrigger, () => check());
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
  color: unset !important;
}
</style>
