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
import { computed, ref, toRefs } from "vue";

import { useCheckable, withCheckableEmit } from "../../composables/Checkable";

import CheckSymbol from "../feedback_symbols/CheckSymbol.vue";
import CrossSymbol from "../feedback_symbols/CrossSymbol.vue";

const propsDef = defineProps<{
  checkTrigger: boolean;
  tiptapNode: Record<string, any>;
}>();
const props = toRefs(propsDef);

const emit = defineEmits({
  ...withCheckableEmit(),
});

const totalPoints = 1;
function check() {
  return value.value === solution.value ? totalPoints : 0;
}
const { right, wrong } = useCheckable(
  props.checkTrigger,
  emit,
  check,
  totalPoints
);

const value = ref("");
const solution = computed(() => props.tiptapNode.value.text);
// Lower resolution to multiples of 5 to not reveal the exact solution length
const width = computed(() => Math.ceil(solution.value.length / 5) * 5);
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
