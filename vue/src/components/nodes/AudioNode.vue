<template>
  <audio controls="true">
    <source :src="sheet.source" :type="sheet.mimetype" />
  </audio>
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
