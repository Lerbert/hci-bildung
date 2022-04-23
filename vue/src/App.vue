<template>
  <div class="container is-fluid content" id="app">
    <edit-sheet-view
      v-if="mode === AppMode.EDIT_SHEET"
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
      :sheet="sheet"
      :sheetTitle="sheettitle"
    ></edit-solution-view>
    <solution-view
      v-else-if="mode === AppMode.VIEW_SOLUTION"
      :sheet="sheet"
      :sheetTitle="sheettitle"
    ></solution-view>
    <demo-view
      v-if="mode === AppMode.DEMO"
      :sheetId="sheetid"
      :initialSheet="sheet"
      :sheetTitle="sheettitle"
    ></demo-view>
  </div>
</template>

<script setup lang="ts">
import { computed, provide, toRefs } from "vue";

import { AppMode } from "./enums";
import { Node, NodeJSON } from "./model/SheetDisplayNode";

import DemoView from "./components/DemoView.vue";
import EditSheetView from "./components/EditSheetView.vue";
import EditSolutionView from "./components/EditSolutionView.vue";
import SheetView from "./components/SheetView.vue";
import SolutionView from "./components/SolutionView.vue";

const propsDef = withDefaults(
  defineProps<{
    mode: AppMode;
    sheetid: string;
    sheettitle?: string;
    content?: NodeJSON;
    saveurl?: string;
  }>(),
  {
    content: () => ({
      type: "doc",
      content: [{ type: "paragraph", content: [], marks: [] }],
      marks: [],
    }),
    sheettitle: "",
    saveurl: "#",
  }
);
const props = toRefs(propsDef);

provide("saveURL", props.saveurl.value);

const sheet = computed(() => Node.fromJSON(props.content.value));
</script>

<style lang="scss" scoped>
input[type="file"]#file-input {
  display: none;
}
</style>
