<template>
  <div>
    <slot></slot>
  </div>
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
