<template>
  <div>
    <div class="py-3">
      <input
        v-model="title"
        class="input is-large has-text-weight-bold"
        :class="{ 'is-danger': title === '' }"
        type="text"
        placeholder="Titel eingeben"
      />
    </div>
    <div class="columns pt-5">
      <div class="column is-half is-flex is-flex-direction-column" id="editor">
        <h1 class="title">Editor</h1>
        <tiptap-editor
          :initialContent="editorContent"
          @update:content="updatePreview($event)"
          :saveStatus="saveStatus"
        ></tiptap-editor>
        <input type="file" id="file-input" accept=".mp3, .ogg, .m4a" />
      </div>
      <div class="column is-half is-flex is-flex-direction-column" id="preview">
        <div class="is-flex is-justify-content-space-between">
          <h1 class="title">Vorschau</h1>
          <div class="is-flex is-justify-content-right buttons">
            <share-button :sheetId="sheetId"></share-button>
            <more-button class="is-right">
              <a class="dropdown-item icon-text" @click="exportDocument">
                <span class="icon">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    width="20"
                    height="20"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z"
                      clip-rule="evenodd"
                    />
                  </svg>
                </span>
                <span>Herunterladen</span>
              </a>
            </more-button>
          </div>
        </div>
        <sheet-display
          :autosave="false"
          :check="true"
          :edit="true"
          :sheet="sheet"
        ></sheet-display>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, toRefs } from "vue";
import { JSONContent } from "@tiptap/core";
import debounce from "lodash/debounce";
import download from "downloadjs";

import { Node } from "../model/SheetDisplayNode";
import { useSaveable } from "../composables/Saveable";

import MoreButton from "./MoreButton.vue";
import ShareButton from "./ShareButton.vue";
import SheetDisplay from "./SheetDisplay.vue";
import TiptapEditor from "./TiptapEditor.vue";

const propsDef = withDefaults(
  defineProps<{
    autosave: boolean;
    sheetId: string;
    initialSheet?: Node;
    sheetTitle?: string;
  }>(),
  {
    initialSheet: () => Node.emptyDocument(),
    sheetTitle: "",
  }
);
const props = toRefs(propsDef);

const editorContent = ref(props.initialSheet.value.toTiptap());
const title = ref(props.sheetTitle.value);
const sheet = computed(() => Node.fromTiptap(editorContent.value));
const doc = computed(() => ({
  title: title.value,
  content: sheet.value,
}));

const { saveStatus } = useSaveable(
  props.autosave,
  doc,
  (doc) => doc.title !== ""
);

const updatePreview = debounce((event: JSONContent) => {
  editorContent.value = event;
}, 100);

function exportDocument() {
  download(
    new Blob([JSON.stringify(doc.value)]),
    title.value + ".json",
    "application/json"
  );
}
</script>

<style lang="scss" scoped>
input[type="file"]#file-input {
  display: none;
}
</style>
