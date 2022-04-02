<template>
  <li>
    <label>
      <input type="checkbox" v-model="ticked" />
      <div><slot /></div>
    </label>
  </li>
</template>

<script setup lang="ts">
import { computed, ref, toRefs, watch } from "vue";

import { useExportable } from "../../composables/Exportable";
import { MultipleChoiceAnswer } from "../../model/SheetDisplayNode";

const propsDef = defineProps<{
  checkTrigger: boolean;
  sheet: MultipleChoiceAnswer;
  sheetExport: MultipleChoiceAnswer;
}>();
const props = toRefs(propsDef);

const emit = defineEmits<{
  (e: "answerCorrect", correct: boolean): void;
}>();

const ticked = ref(props.sheet.value.answer);
const checked = ref(false);
const correct = ref(false);
const solution = computed(() => props.sheet.value.solution);

function check() {
  checked.value = true;
  correct.value = ticked.value === solution.value;
  emit("answerCorrect", correct.value);
}

watch(props.checkTrigger, check);

function updateExport() {
  props.sheetExport.value.answer = ticked.value;
}

useExportable(props.sheet, updateExport, [ticked]);
</script>

<style lang="scss" scoped>
label {
  align-items: center;
  display: flex;

  > input {
    flex: 0 0 auto;
    margin-right: 0.5rem;
  }

  > div {
    flex: 1 1 auto;
    min-width: 0%;
  }
}
</style>
