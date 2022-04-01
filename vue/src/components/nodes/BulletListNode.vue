<template>
  <ul>
    <slot />
  </ul>
</template>

<script setup lang="ts">
import { Node } from "../../model/SheetDisplayNode";

const props = defineProps<{
  checkTrigger: boolean;
  sheet: Node;
  sheetExport: Node;
}>();
</script>

<style></style>
