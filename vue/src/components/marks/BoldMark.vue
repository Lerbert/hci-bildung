<template>
  <strong>
    <slot></slot>
  </strong>
</template>

<script setup lang="ts">
import { Mark } from "../../model/SheetDisplayMark";

defineProps<{
  checkTrigger: boolean;
  mark: Mark;
  markExport: Mark;
}>();
</script>

<style></style>
