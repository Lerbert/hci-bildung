<template>
  <span>{{ sheet.text }}</span>
</template>

<script setup lang="ts">
import { Text } from "../../model/SheetDisplayNode";

defineProps<{
  checkTrigger: boolean;
  sheet: Text;
  sheetExport: Text;
}>();
</script>

<style></style>
