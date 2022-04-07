<template>
  <div>
    <div class="is-flex is-justify-content-space-between">
      <h1>{{ sheetTitle }}</h1>
      <form :action="`${sheetId}/solve`" method="POST">
        <button class="button is-link" type="submit">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            width="24"
            height="24"
            stroke-width="2"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M11 4a2 2 0 114 0v1a1 1 0 001 1h3a1 1 0 011 1v3a1 1 0 01-1 1h-1a2 2 0 100 4h1a1 1 0 011 1v3a1 1 0 01-1 1h-3a1 1 0 01-1-1v-1a2 2 0 10-4 0v1a1 1 0 01-1 1H7a1 1 0 01-1-1v-3a1 1 0 00-1-1H4a2 2 0 110-4h1a1 1 0 001-1V7a1 1 0 011-1h3a1 1 0 001-1V4z"
            />
          </svg>
          &nbsp; Lösung beginnen
        </button>
      </form>
    </div>
    <sheet-display :edit="false" :sheet="sheet"></sheet-display>
  </div>
</template>

<script setup lang="ts">
import { Node } from "../model/SheetDisplayNode";

import SheetDisplay from "./SheetDisplay.vue";

defineProps<{
  sheetId: string;
  sheet: Node;
  sheetTitle: string;
}>();
</script>

<style></style>
