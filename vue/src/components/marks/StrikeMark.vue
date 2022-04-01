<template>
  <s><slot /></s>
</template>

<script setup lang="ts">
import { Mark } from "../../model/SheetDisplayMark";

const props = defineProps<{
  checkTrigger: boolean;
  mark: Mark;
  markExport: Mark;
}>();
</script>

<style></style>
