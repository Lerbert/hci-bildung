<template>
  <pre>
    <code v-bind:class="tiptapNode.attrs.language == null ? '' : 'language-' + tiptapNode.attrs.language">
      <slot/>
    </code>
  </pre>
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
