<template>
  <audio controls="true">
    <source :src="sheet.source" :type="sheet.mimetype" />
  </audio>
</template>

<script setup lang="ts">
import { Audio } from "../../model/SheetDisplayNode";

defineProps<{
  checkTrigger: boolean;
  sheet: Audio;
  sheetExport: Audio;
}>();
</script>

<style></style>
