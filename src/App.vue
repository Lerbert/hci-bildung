<template>
  <div id="app">
    <tiptap v-model="editorContent"></tiptap>
    <p>Sicht der Lernenden</p>
    <sheet :editorHTML="editorContent"></sheet>
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

