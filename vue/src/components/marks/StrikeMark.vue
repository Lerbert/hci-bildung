<template>
  <s>
    <slot></slot>
  </s>
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
