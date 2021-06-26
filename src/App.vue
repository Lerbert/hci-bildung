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
      editorContent: {type: 'doc', content: [{type: 'paragraph'}]},
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