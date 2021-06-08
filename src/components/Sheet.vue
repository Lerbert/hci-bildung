<template>
  <!-- <div v-html="studentView"></div> -->
  <node :tiptapNode="editorJSON"></node>
  <button v-on:click="checkAll">Alle überprüfen</button>
  <div>{{ editorJSON }}</div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Node from './nodes/Node.vue';


export default defineComponent({
  components: {
    Node,
  },

  props: {
    editorJSON: {
      type: Object,
      required: true,
    },
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
});
</script>

<style lang="scss">
  input.correct-gap {
    background-color: rgba(#00ff00, 0.5);
  }
  input.incorrect-gap {
    background-color: rgba(#ff0000, 0.5);
  }
</style>