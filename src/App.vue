<template>
  <div class="container is-fluid content" id="app">
    <div class="py-3"><input class="input is-large has-text-weight-bold" type="text" value="Neues Dokument"></div>
    <div class="columns pt-5">
      <div class="column is-half" id="editor">
        <h1 class="title">Editor</h1>
        <tiptap :initialContent="editorContent" @update:content="editorContent=$event"></tiptap>
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

<script lang="ts">
import { defineComponent } from 'vue';
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
    }
  },
});

</script>

<style lang="scss" scoped>
  input[type="file"]#file-input {
    display: none;
  }

  div#preview > div {
    padding-top: 46px;
  }
</style>