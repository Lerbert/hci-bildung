<template>
  <p>
    <slot />
    <br v-if="sheet.content.length === 0" />
  </p>
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
