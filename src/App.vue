<template>
  <div class="container is-fluid content" id="app">
    <div class="py-3"><input class="input is-large has-text-weight-bold" type="text" placeholder="Titel eingeben"></div>
    <div class="columns pt-5">
      <div class="column is-half" id="editor">
        <h1 class="title">Editor</h1>
        <tiptap :initialContent="editorContent" @update:content="handleContentUpdate($event)" :saveStatus="saveStatus"></tiptap>
        <input type="file" id="file-input" accept=".mp3, .ogg, .m4a">
      </div>
      <div class="column is-half" id="preview">
        <div class="is-flex is-justify-content-space-between">
          <h1 class="title">Vorschau</h1>
          <share-button class=""></share-button>
        </div>
        <sheet :editorJSON="editorContent"></sheet>
      </div>
    </div>
    <!-- <span>{{editorContent}}</span> -->
  </div>
</template>

<script>
import { defineComponent } from 'vue';
import debounce from 'lodash/debounce';

import { SaveStatus } from './enums.ts';
import Tiptap from './components/Tiptap.vue';
import Sheet from './components/Sheet.vue';
import ShareButton from './components/ShareButton.vue';

export default defineComponent({
  name: 'App',
  components: {
    ShareButton,
    Sheet,
    Tiptap,
  },

  data() {
    return {
      // editorContent: {type: 'doc', content: [{type: 'paragraph'}]},
      editorContent: { "type": "doc", "content": [ { "type": "heading", "attrs": { "level": 1 }, "content": [ { "type": "text", "marks": [ { "type": "bold" } ], "text": "Listening Comprehension" } ] }, { "type": "paragraph", "content": [ { "type": "text", "text": "You will " }, { "type": "text", "marks": [ { "type": "bold" } ], "text": "hear a podcast" }, { "type": "text", "text": " about Instagram, the photo-sharing social networking site, and some of its unexpected consequences." } ] }, { "type": "paragraph", "content": [ { "type": "text", "text": "1. " }, { "type": "text", "marks": [ { "type": "italic" } ], "text": "Complete the information." }, { "type": "text", "text": " Brent Knepper…" } ] }, { "type": "bulletList", "content": [ { "type": "listItem", "content": [ { "type": "paragraph", "content": [ { "type": "text", "text": "wrote an article called “Instagram is _______________________________”" } ] } ] }, { "type": "listItem", "content": [ { "type": "paragraph", "content": [ { "type": "text", "text": "works as a t____________ _____________" } ] } ] }, { "type": "listItem", "content": [ { "type": "paragraph", "content": [ { "type": "text", "text": "calls in from ________________________________" } ] } ] } ] }, { "type": "paragraph", "content": [ { "type": "text", "text": "2. Horseshoe Bend. " }, { "type": "text", "marks": [ { "type": "italic" } ], "text": "Complete the sentences." } ] }, { "type": "paragraph", "content": [ { "type": "text", "text": "One of Knepper’s examples is Horseshoe Bend in __________________________________. In two ways, it is a very special place because firstly the river makes a __________________________________________ and secondly the canyon is ___________________________." } ] }, { "type": "paragraph" }, { "type": "paragraph", "content": [ { "type": "text", "text": "Source: https://www.isb.bayern.de/download/22676/jahrgangsstufentest_englisch_2019___jgst_10_aufgaben.pdf" } ] } ] },
      saveStatus: SaveStatus.SAVED,
      updatePreview: debounce((event) => {
        this.editorContent = event;
      }, 100),
      save: debounce(async () => {
        this.saveStatus = SaveStatus.SAVING;
        await new Promise(r => setTimeout(r, 1000));
        if (this.saveStatus === SaveStatus.SAVING) {
          this.saveStatus = SaveStatus.SAVED;
        }
      }, 500),
    }
  },

  methods: {
    handleContentUpdate(event) {
      this.saveStatus = SaveStatus.WAITING;
      this.updatePreview(event);
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