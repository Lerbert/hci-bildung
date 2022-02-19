<template>
  <div>
    <h1>{{ docTitle }}</h1>
    <sheet-display :editorJSON="docJSON"></sheet-display>
  </div>
</template>

<script>
import { defineComponent } from "vue";

import SheetDisplay from "./SheetDisplay.vue";

export default defineComponent({
  name: "EditView",
  components: {
    SheetDisplay,
  },

  props: {
    docId: {
      type: String,
      required: true,
    },
    docJSON: {
      type: Object,
      required: true,
    },
    docTitle: {
      type: String,
      required: true,
    },
  },
});
</script>

<style>
</style>