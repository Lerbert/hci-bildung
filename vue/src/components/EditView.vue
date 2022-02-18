<template>
  <div>
    <div class="py-3">
      <input
        v-model="title"
        class="input is-large has-text-weight-bold"
        type="text"
        placeholder="Titel eingeben"
      />
    </div>
    <div class="columns pt-5">
      <div class="column is-half" id="editor">
        <h1 class="title">Editor</h1>
        <tiptap
          :initialContent="editorContent"
          @update:content="handleContentUpdate($event)"
          :saveStatus="saveStatus"
        ></tiptap>
        <input type="file" id="file-input" accept=".mp3, .ogg, .m4a" />
      </div>
      <div class="column is-half" id="preview">
        <div class="is-flex is-justify-content-space-between">
          <h1 class="title">Vorschau</h1>
          <share-button class=""></share-button>
        </div>
        <sheet :editorJSON="editorContent"></sheet>
      </div>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import debounce from "lodash/debounce";

import { SaveStatus } from "../enums";
import Tiptap from "./Tiptap.vue";
import Sheet from "./Sheet.vue";
import ShareButton from "./ShareButton.vue";

export default defineComponent({
  name: "EditView",
  components: {
    ShareButton,
    Sheet,
    Tiptap,
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

  methods: {
    handleContentUpdate(event) {
      this.saveStatus = SaveStatus.WAITING;
      this.updatePreview(event);
      this.save();
    },
    async saveHelper() {
      this.saveStatus = SaveStatus.SAVING;
      let delay = new Promise((r) => setTimeout(r, 500)); // Delay to show user that we are indeed saving
      let saveOk = false;
      try {
        let response = await fetch(this.docId, {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            id: this.docId,
            title: this.title,
            tiptap: this.editorContent,
          }),
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