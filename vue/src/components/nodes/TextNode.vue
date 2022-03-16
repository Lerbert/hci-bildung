<template>
  <span>{{ tiptapNode.text }}</span>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    tiptapNode: {
      type: Object,
      required: true,
    },
  },
});
</script>

<style></style>
