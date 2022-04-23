<template>
  <div>
    <sheet-editor
      :autosave="false"
      :sheetId="'00000000-0000-0000-0000-000000000000'"
      :sheetTitle="'Demo'"
      :initialSheet="initialSheet"
    ></sheet-editor>
  </div>
</template>

<script setup lang="ts">
import { Node } from "../model/SheetDisplayNode";

import SheetEditor from "./SheetEditor.vue";

const initialSheet = Node.emptyDocument();
</script>

<style></style>
