<template>
  <li><slot /></li>
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
