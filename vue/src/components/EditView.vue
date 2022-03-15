<template>
  <div>
    <div class="py-3">
      <input
        v-model="title"
        class="input is-large has-text-weight-bold"
        :class="title === '' ? 'is-danger' : ''"
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
            <share-button :docId="docId"></share-button>
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
        <sheet-display :editorJSON="editorContent"></sheet-display>
      </div>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import debounce from "lodash/debounce";
import download from "downloadjs";

import MoreButton from "./MoreButton.vue";
import { SaveStatus } from "../enums";
import ShareButton from "./ShareButton.vue";
import SheetDisplay from "./SheetDisplay.vue";
import TiptapEditor from "./TiptapEditor.vue";

export default defineComponent({
  name: "EditView",
  components: {
    MoreButton,
    ShareButton,
    SheetDisplay,
    TiptapEditor,
  },

  props: {
    docId: {
      type: String,
      required: true,
    },
    docJSON: {
      type: Object,
      default: () => ({ type: "doc", content: [{ type: "paragraph" }] }),
    },
    docTitle: {
      type: String,
      default: "",
    },
  },

  data() {
    return {
      editorContent: this.docJSON,
      title: this.docTitle,
      saveStatus: SaveStatus.SAVED,
      updatePreview: debounce((event) => {
        this.editorContent = event;
      }, 100),
      save: debounce(this.saveHelper, 1000),
      saveBackoff: 0,
    };
  },

  computed: {
    doc() {
      return {
        title: this.title,
        tiptap: this.editorContent,
      };
    },
  },

  methods: {
    handleContentUpdate(event) {
      this.saveStatus = SaveStatus.WAITING;
      this.updatePreview(event);
      this.save();
    },
    async saveHelper() {
      if (this.title === "") {
        this.saveStatus = SaveStatus.FAILED;
        return;
      }
      this.saveStatus = SaveStatus.SAVING;
      let delay = new Promise((r) => setTimeout(r, 500)); // Delay to show user that we are indeed saving
      let saveOk = false;
      try {
        let response = await fetch(this.docId, {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(this.doc),
        });
        await delay;
        saveOk = response.status === 200;
      } catch (e) {
        console.log("Error while saving:", e);
        saveOk = false;
      }
      if (this.saveStatus === SaveStatus.SAVING) {
        if (saveOk) {
          this.saveStatus = SaveStatus.SAVED;
          this.saveBackoff = 0;
        } else if (this.saveBackoff >= 5000) {
          this.saveStatus = SaveStatus.FAILED;
          console.log("Saving failed. No further automatic retries");
        } else {
          this.saveBackoff += 1000;
          console.log(
            "Saving failed. Retrying automatically in",
            this.saveBackoff / 1000,
            "s"
          );
          setTimeout(this.save, this.saveBackoff);
        }
      }
    },
    exportDocument() {
      download(JSON.stringify(this.doc), this.title, "application/json");
    },
  },

  watch: {
    title: function () {
      this.save();
    },
  },
});
</script>

<style lang="scss" scoped>
input[type="file"]#file-input {
  display: none;
}
</style>
