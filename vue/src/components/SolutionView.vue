<template>
  <div>
    <h1>{{ sheetTitle }}</h1>
    <sheet-display
      :autosave="false"
      :check="true"
      :edit="false"
      :sheet="sheet"
    ></sheet-display>
  </div>
</template>

<script setup lang="ts">
import { Node } from "../model/SheetDisplayNode";

import SheetDisplay from "./SheetDisplay.vue";

defineProps<{
  sheetId: string;
  sheet: Node;
  sheetTitle: string;
}>();
</script>

<style></style>
