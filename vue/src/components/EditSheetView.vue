<template>
  <div>
    <sheet-editor
      :autosave="true"
      :sheetId="sheetId"
      :sheetTitle="sheetTitle"
      :initialSheet="initialSheet"
    ></sheet-editor>
  </div>
</template>

<script setup lang="ts">
import { Node } from "../model/SheetDisplayNode";

import SheetEditor from "./SheetEditor.vue";

withDefaults(
  defineProps<{
    sheetId: string;
    initialSheet?: Node;
    sheetTitle?: string;
  }>(),
  {
    initialSheet: () => Node.emptyDocument(),
    sheetTitle: "",
  }
);
</script>

<style></style>
