<template>
  <audio controls="true">
    <source :src="sheet.source" :type="sheet.mimetype" />
  </audio>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    sheet: {
      type: Object,
      required: true,
    },
  },
});
</script>

<style></style>
