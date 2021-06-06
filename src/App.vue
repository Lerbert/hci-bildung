<template>
  <div id="app">
    <tiptap v-model="editorContent"></tiptap>
    Inhalt: {{ editorContent }}
  </div>
</template>

<script>
import Tiptap from './components/Tiptap.vue'

export default {
  name: 'App',
  
  components: {
    Tiptap
  },

  data() {
    return {
      editorContent: '',
    }
  },
}
</script>
