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
          @update:content="handleContentUpdate($event)"
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
        <sheet-display :sheet="sheet"></sheet-display>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, toRefs, watch } from "vue";
import { JSONContent } from "@tiptap/core";
import debounce from "lodash/debounce";
import download from "downloadjs";

import { Node } from "../model/SheetDisplayNode";
import { SaveStatus } from "../enums";

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

const saveStatus = ref(
  props.autosave.value ? SaveStatus.SAVED : SaveStatus.DISABLED
);
const saveBackoff = ref(0);
const save = debounce(() => {
  if (props.autosave.value) {
    saveHelper();
  }
}, 1000);
async function saveHelper() {
  if (title.value === "") {
    saveStatus.value = SaveStatus.FAILED;
    return;
  }
  saveStatus.value = SaveStatus.SAVING;
  let delay = new Promise((r) => setTimeout(r, 500)); // Delay to show user that we are indeed saving
  let saveOk = false;
  try {
    let response = await fetch(".", {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(doc.value),
    });
    await delay;
    saveOk = response.status === 200;
  } catch (e) {
    console.log("Error while saving:", e);
    saveOk = false;
  }
  if (saveStatus.value === SaveStatus.SAVING) {
    if (saveOk) {
      saveStatus.value = SaveStatus.SAVED;
      saveBackoff.value = 0;
    } else if (saveBackoff.value >= 5000) {
      saveStatus.value = SaveStatus.FAILED;
      console.log("Saving failed. No further automatic retries");
    } else {
      saveBackoff.value += 1000;
      console.log(
        "Saving failed. Retrying automatically in",
        saveBackoff.value / 1000,
        "s"
      );
      setTimeout(save, saveBackoff.value);
    }
  }
}
watch(title, () => save());

const updatePreview = debounce((event: JSONContent) => {
  editorContent.value = event;
}, 100);
function handleContentUpdate(event: JSONContent) {
  if (props.autosave.value) {
    saveStatus.value = SaveStatus.WAITING;
  }
  updatePreview(event);
  save();
}

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
