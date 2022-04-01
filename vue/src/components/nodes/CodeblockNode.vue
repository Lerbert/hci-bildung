<template>
  <pre>
    <code :class="language">
      <slot/>
    </code>
  </pre>
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

  computed: {
    language(): string {
      return this.sheet.language === ""
        ? ""
        : `language-${this.sheet.language}`;
    },
  },
});
</script>

<style></style>
