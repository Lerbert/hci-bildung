<template>
  <div class="sheet">
    <div class="sheet__header">
      <button
        class="button is-success is-small"
        v-on:click="checkAll"
        :disabled="!edit"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-6 w-6"
          fill="none"
          viewBox="0 0 24 24"
          width="24"
          height="24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"
          />
        </svg>
        &nbsp; Alle überprüfen
      </button>
    </div>
    <sheet-node
      class="sheet__content"
      :sheet="sheet"
      :sheetExport="sheetExport"
      :checkTrigger="checkTrigger"
      @grantPoints="grantPoints"
    ></sheet-node>
    <div class="sheet__footer">
      <point-status
        :achievedPoints="achievedPoints"
        :totalPoints="totalPoints"
      ></point-status>
      <save-status
        :saveStatus="status"
        :hide-when-disabled="true"
      ></save-status>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, provide, ref, toRefs, watch } from "vue";
import cloneDeep from "lodash/cloneDeep";
import debounce from "lodash/debounce";

import { Node } from "../model/SheetDisplayNode";
import { useSaveable } from "../composables/Saveable";

import SaveStatus from "./SaveStatus.vue";
import SheetNode from "./nodes/SheetNode.vue";
import PointStatus from "./PointStatus.vue";

const propsDef = defineProps<{
  autosave: boolean;
  edit: boolean;
  sheet: Node;
}>();
const props = toRefs(propsDef);

const emit = defineEmits<{
  (e: "update:export", sheet: Node): void;
}>();

provide("edit", props.edit);

const checkTrigger = ref(false);
const achievedPoints = ref(0);
const totalPoints = ref(0);

function checkAll() {
  achievedPoints.value = 0;
  totalPoints.value = 0;
  checkTrigger.value = !checkTrigger.value;
}

function grantPoints(event: { achievedPoints: number; totalPoints: number }) {
  achievedPoints.value += event.achievedPoints;
  totalPoints.value += event.totalPoints;
}

// A note on sheetExport:
// We use this as a prop for the child components which will mutate it to make their information (e.g. the answer to a gap) available to this component.
// Aware that this is a bad practice, we chose this approach among the following alternatives:
// - Use events to pass the information from the child to the parent: The problem here is making sure that the events are processed in the correct order and knowing which part of the state to mutate in the parent.
// - Use a global store: Problematic, because each sheet we want to display needs its own store.
// This solution can be considered as instantiating a "global store" for each instance of this component and passing the reference to the store to the child components.
// Since those will muatate the store's content, we run into the bad practice mentioned above.
// However, the mark and node components are intended to only be used inside of this component, thus making the problem less severe (see https://vuejs.org/guide/components/props.html#mutating-object-array-props).
// See also https://forum.vuejs.org/t/data-sync-on-deeply-nested-structures/40099 for a similar approach
const sheetExport = ref(cloneDeep(props.sheet.value));
watch(props.sheet, () => (sheetExport.value = cloneDeep(props.sheet.value)));

const updateExport = debounce(() => {
  emit("update:export", cloneDeep(sheetExport.value));
}, 100);
watch(sheetExport, updateExport, { deep: true });

const solution = computed(() => ({
  content: sheetExport.value,
}));
const { saveStatus: status } = useSaveable(
  "./my",
  props.autosave,
  solution,
  () => true
);
</script>

<style lang="scss">
.sheet {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  max-height: 600px;
  color: #0d0d0d;
  background-color: #ffffff;
  border: 3px solid #0d0d0d;
  border-radius: 0.75rem;

  &__header {
    display: flex;
    align-items: center;
    flex: 0 0 auto;
    flex-wrap: wrap;
    padding: 0.25rem;
    padding-bottom: 0.1875rem;
    border-bottom: 3px solid #0d0d0d;
    & > {
      height: 1.75rem;
      padding: 0.25rem;
      margin-right: 0.25rem;
    }
  }

  &__content {
    padding: 0.5rem;
    flex: 1 1 auto;
    overflow-x: hidden;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    word-wrap: break-word;
    white-space: pre-wrap;
    white-space: break-spaces;
    -webkit-font-variant-ligatures: none;
    font-variant-ligatures: none;
    font-feature-settings: "liga" 0;
  }

  &__footer {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    white-space: nowrap;
    border-top: 3px solid #0d0d0d;
    font-size: 12px;
    font-weight: 600;
    color: #0d0d0d;
    white-space: nowrap;
    padding: 0.25rem 0.75rem;
  }
}

.sheet__content {
  > * + * {
    margin-top: 0.75em;
  }

  ul,
  ol {
    padding: 0 1rem;
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    line-height: 1.1;
  }

  code {
    background-color: rgba(#616161, 0.1);
    color: #616161;
  }

  pre {
    background: #0d0d0d;
    color: #fff;
    font-family: "JetBrainsMono", monospace;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    white-space: pre-wrap;

    code {
      color: inherit;
      padding: 0;
      background: none;
      font-size: 0.8rem;
    }
  }

  img {
    max-width: 100%;
    height: auto;
  }

  span[data-type="gap"] {
    color: #ffffff;
    background-color: #3d3d3d;
  }

  .correct {
    background-color: rgba(#48c774, 0.1);
    border-color: #48c774;
  }
  .incorrect {
    background-color: rgba(#f14668, 0.1);
    border-color: #f14668;
  }
}
</style>
