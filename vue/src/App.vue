<template>
  <div class="container is-fluid content" id="app">
    <edit-view v-if="edit" :docJSON="docJSON" :docTitle="docTitle"></edit-view>
    <student-view v-if="edit" :docJSON="docJSON" :docTitle="docTitle"></student-view>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import EditView from './components/EditView.vue';
import StudentView from './components/StudentView.vue'

export default defineComponent({
  name: 'App',
  components: {
    EditView,
    StudentView,
  },

  data() {
    return {
      docJSON: { "type": "doc", "content": [ { "type": "heading", "attrs": { "level": 2 }, "content": [ { "type": "text", "text": "Überschrift" } ] }, { "type": "paragraph", "content": [ { "type": "text", "text": "Aufgabentext." } ] }, { "type": "paragraph", "content": [ { "type": "text", "text": "Das ist eine " }, { "type": "text", "marks": [ { "type": "gap" } ], "text": "Lücke" }, { "type": "text", "text": "." } ] } ] }, // {type: 'doc', content: [{type: 'paragraph'}]},
      docTitle: "Amazing Title",
      edit: true,
    }
  },
});

</script>

<style lang="scss" scoped>
  input[type="file"]#file-input {
    display: none;
  }
</style>