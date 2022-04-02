<template>
  <pre>
    <code :class="language">
      <slot></slot>
    </code>
  </pre>
</template>

<script setup lang="ts">
import { computed, toRefs } from "vue";

import { Codeblock } from "../../model/SheetDisplayNode";

const propsDef = defineProps<{
  checkTrigger: boolean;
  sheet: Codeblock;
  sheetExport: Codeblock;
}>();
const props = toRefs(propsDef);

const language = computed(() => `language-${props.sheet.value.language}`);
</script>

<style></style>
