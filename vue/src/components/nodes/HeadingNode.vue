<template>
  <component :is="`h${sheet.level}`">
    <slot></slot>
    <br v-if="sheet.content.length === 0" />
  </component>
</template>

<script setup lang="ts">
import { Heading } from "../../model/SheetDisplayNode";

defineProps<{
  checkTrigger: boolean;
  sheet: Heading;
  sheetExport: Heading;
}>();
</script>

<style></style>
