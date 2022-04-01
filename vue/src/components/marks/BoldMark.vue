<template>
  <strong><slot /></strong>
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
