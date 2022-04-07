<template>
  <div class="container is-fluid content" id="app">
    <edit-sheet-view
      v-if="mode === AppMode.EDIT_SHEET"
      :autosave="true"
      :sheetId="sheetid"
      :initialSheet="sheet"
      :sheetTitle="sheettitle"
    ></edit-sheet-view>
    <sheet-view
      v-else-if="mode === AppMode.VIEW_SHEET"
      :sheetId="sheetid"
      :sheet="sheet"
      :sheetTitle="sheettitle"
    ></sheet-view>
    <edit-solution-view
      v-else-if="mode === AppMode.EDIT_SOLUTION"
      :sheetId="sheetid"
      :sheet="sheet"
      :sheetTitle="sheettitle"
    ></edit-solution-view>
  </div>
</template>

<script setup lang="ts">
import { computed, toRefs } from "vue";

import { AppMode } from "./enums";
import { Node, NodeJSON } from "./model/SheetDisplayNode";

import EditSheetView from "./components/EditSheetView.vue";
import EditSolutionView from "./components/EditSolutionView.vue";
import SheetView from "./components/SheetView.vue";

const propsDef = withDefaults(
  defineProps<{
    sheetid: string;
    content?: NodeJSON;
    sheettitle?: string;
    mode: AppMode;
  }>(),
  {
    content: () => ({
      type: "doc",
      content: [{ type: "paragraph", content: [], marks: [] }],
      marks: [],
    }),
    sheettitle: "",
  }
);
const props = toRefs(propsDef);

const sheet = computed(() => Node.fromJSON(props.content.value));
</script>

<style lang="scss" scoped>
input[type="file"]#file-input {
  display: none;
}
</style>
