<template>
  <div v-html="studentView"></div>
  <button v-on:click="checkAll">Alle überprüfen</button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    editorHTML: {
      type: String,
      required: true,
    },
  },

  data() {
    return {
      studentView: '',
    }
  },

  methods: {
    checkAll() {
      for (const i of Array.from(document.getElementsByTagName("input"))) {
        if (i.getAttribute("data-type") == "gap") {
          const solution = i.getAttribute("data-solution");
          const value = i.value;
          if (solution === value) {
            i.classList.remove("incorrect-gap");
            i.classList.add("correct-gap");
          } else {
            i.classList.remove("correct-gap");
            i.classList.add("incorrect-gap");
          }
        }
      }
    }
  },

  watch: {
    editorHTML(val) {
      const parser = new DOMParser();
      let sheet = parser.parseFromString(val, "text/html");
      for (const s of Array.from(sheet.getElementsByTagName("span"))) {
        if (s.getAttribute("data-type") == "gap") {
          const i = sheet.createElement("input");
          i.setAttribute("type", "text");
          i.setAttribute("data-type", "gap");
          i.setAttribute("data-solution", s.innerText);
          s.replaceWith(i);
        }
      }
      this.studentView = sheet.documentElement.innerHTML;
    }
  }
});
</script>

<style>

</style>