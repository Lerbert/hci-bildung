<template>
  <div>
    <h1>{{ docTitle }}</h1>
    <sheet-display :sheet="sheet"></sheet-display>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { Node } from "../model/SheetDisplayNode";

import SheetDisplay from "./SheetDisplay.vue";

export default defineComponent({
  name: "StudentView",
  components: {
    SheetDisplay,
  },

  props: {
    docId: {
      type: String,
      required: true,
    },
    sheet: {
      type: Node,
      required: true,
    },
    docTitle: {
      type: String,
      required: true,
    },
  },
});
</script>

<style></style>
