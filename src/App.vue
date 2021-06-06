<template>
  <div id="app">
    <tiptap v-model="editorContent"></tiptap>
    <p>Sicht der Lernenden</p>
    <div v-html="sheet"></div>
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
          s.replaceWith(i);
        }
      }
      this.sheet = sheet.documentElement.innerHTML;
    }
  }
});
</script>

