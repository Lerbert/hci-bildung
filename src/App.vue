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
    }
  },
});
</script>

