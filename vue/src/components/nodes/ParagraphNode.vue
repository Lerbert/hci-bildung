<template>
  <p>
    <slot />
    <br v-if="sheet.content.length === 0" />
  </p>
</template>

<script setup lang="ts">
import { Node } from "../../model/SheetDisplayNode";

defineProps<{
  checkTrigger: boolean;
  sheet: Node;
  sheetExport: Node;
}>();
</script>

<style></style>
