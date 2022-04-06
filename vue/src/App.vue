<template>
  <div class="container is-fluid content" id="app">
    <edit-view
      v-if="edit"
      :docId="docid"
      :initialSheet="sheet"
      :docTitle="doctitle"
    ></edit-view>
    <student-view
      v-else
      :docId="docid"
      :sheet="sheet"
      :docTitle="doctitle"
    ></student-view>
  </div>
</template>

<script setup lang="ts">
import { computed, toRefs } from "vue";

import { Node, NodeJSON } from "./model/SheetDisplayNode";

import EditView from "./components/EditView.vue";
import StudentView from "./components/StudentView.vue";

const propsDef = withDefaults(
  defineProps<{
    docid: string;
    docjson?: NodeJSON;
    doctitle?: string;
    edit: boolean;
  }>(),
  {
    docjson: () => ({
      type: "doc",
      content: [{ type: "paragraph", content: [], marks: [] }],
      marks: [],
    }),
    doctitle: "",
  }
);
const props = toRefs(propsDef);

const sheet = computed(() => Node.fromJSON(props.docjson.value));
</script>

<style lang="scss" scoped>
input[type="file"]#file-input {
  display: none;
}
</style>
