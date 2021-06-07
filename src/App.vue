<template>
  <div id="app">
    <input type="file" id="file-input" accept=".mp3, .ogg, .m4a">
    <div id="editor">
      <h1>Editor</h1>
      <tiptap v-model="editorContent"></tiptap>
    </div>
    <div id="preview">
      <h1>Sicht der Lernenden</h1>
      <div v-html="sheet"></div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Tiptap from './components/Tiptap.vue';

export default defineComponent({
  name: 'App',
  components: {
    Tiptap,
  },

  data() {
    return {
      editorContent: '',
      sheet: '',
    }
  },

  watch: {
    editorContent: function(val) {
      const parser = new DOMParser();
      let sheet = parser.parseFromString(val, "text/html");
      for (const s of Array.from(sheet.getElementsByTagName("span"))) {
        if (s.getAttribute("data-type") == "gap") {
          const i = sheet.createElement("input");
          i.setAttribute("type", "text");
          i.setAttribute("data-solution", s.innerText);
          s.replaceWith(i);
        }
      }
      this.sheet = sheet.documentElement.innerHTML;
    }
  }
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