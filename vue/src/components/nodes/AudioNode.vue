<template>
  <audio controls="true">
    <source
      v-bind:src="tiptapNode.attrs.source"
      v-bind:type="tiptapNode.attrs.mimetype"
    />
  </audio>
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
