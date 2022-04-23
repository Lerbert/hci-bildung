<template>
  <div>
    <h1>{{ sheetTitle }}</h1>
    <sheet-display
      :autosave="true"
      :check="true"
      :edit="true"
      :sheet="sheet"
    ></sheet-display>
  </div>
</template>

<script setup lang="ts">
import { Node } from "../model/SheetDisplayNode";

import SheetDisplay from "./SheetDisplay.vue";

defineProps<{
  sheet: Node;
  sheetTitle: string;
}>();
</script>

<style></style>
