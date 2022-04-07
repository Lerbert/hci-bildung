<template>
  <div class="container is-fluid content" id="app">
    <edit-view
      v-if="mode === AppMode.EDIT_SHEET"
      :sheetId="sheetid"
      :initialSheet="sheet"
      :sheetTitle="sheettitle"
    ></edit-view>
    <student-view
      v-else-if="mode === AppMode.VIEW_SHEET"
      :sheetId="sheetid"
      :sheet="sheet"
      :sheetTitle="sheettitle"
    ></student-view>
  </div>
</template>

<script setup lang="ts">
import { computed, toRefs } from "vue";

import { AppMode } from "./enums";
import { Node, NodeJSON } from "./model/SheetDisplayNode";

import EditView from "./components/EditView.vue";
import StudentView from "./components/StudentView.vue";

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
