<template>
  <span>{{ sheet.text }}</span>
</template>

<script setup lang="ts">
import { Text } from "../../model/SheetDisplayNode";

const props = defineProps<{
  checkTrigger: boolean;
  sheet: Text;
  sheetExport: Text;
}>();
</script>

<style></style>
