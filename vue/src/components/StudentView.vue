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
    docJSON: {
      type: Object,
      required: true,
    },
    docTitle: {
      type: String,
      required: true,
    },
  },

  computed: {
    sheet() {
      return Node.fromTiptap(this.docJSON);
    },
  },
});
</script>

<style></style>
