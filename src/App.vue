<template>
  <div id="app">
    <input type="file" id="file-input" accept=".mp3, .ogg, .m4a">
    <div id="editor">
      <h1>Editor</h1>
      <tiptap v-model="editorContent"></tiptap>
    </div>
    <div id="preview">
      <h1>Sicht der Lernenden</h1>
      <sheet :editorJSON="editorContent"></sheet>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Tiptap from './components/Tiptap.vue';
import Sheet from './components/Sheet.vue';

export default defineComponent({
  name: 'App',
  components: {
    Tiptap,
    Sheet,
  },

  data() {
    return {
      editorContent: {type:'doc'},
    }
  },
});

</script>

<style lang="scss" scoped>
  input[type="file"]#file-input {
    display: none;
  }
  
  div#app {
    display: flex;
  }

  div#editor {
    padding: 10px;
    width: 50%;
  }

  div#preview {
    padding: 10px;
    width: 50%;
  }

  div#preview > div {
    padding-top: 46px;
  }
</style>