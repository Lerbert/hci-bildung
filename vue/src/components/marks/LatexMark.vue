<template>
  <span ref="latex"></span>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, toRefs, watch } from "vue";
import Katex from "katex";

import { Latex } from "../../model/SheetDisplayMark";

const propsDef = defineProps<{
  checkTrigger: boolean;
  mark: Latex;
  markExport: Latex;
}>();
const props = toRefs(propsDef);

const latex = ref<HTMLSpanElement | null>(null);
const latexSource = computed(() => props.mark.value.source);

function renderLatex() {
  if (latex.value !== null) {
    Katex.render(latexSource.value, latex.value, {
      throwOnError: false,
    });
  }
}

onMounted(renderLatex);

watch(latexSource, renderLatex);
</script>

<style lang="css" scoped>
@import "../../assets/katex/katex.min.css";
</style>
