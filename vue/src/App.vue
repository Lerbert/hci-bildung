<template>
  <div class="container is-fluid content" id="app">
    <edit-view v-if="edit" :docJSON="docjson" :docTitle="doctitle"></edit-view>
    <student-view v-else :docJSON="docjson" :docTitle="doctitle"></student-view>
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

  props: {
      docid: {
        type: String,
        required: true,
      },
      docjson: {
        type: Object,
        default: () => ({type: 'doc', content: [{type: 'paragraph'}]}),
      },
      doctitle: {
        type: String,
        default: "",
      },
      edit: {
        type: Boolean,
        required: true,
      },
  },
});

</script>

<style lang="scss" scoped>
  input[type="file"]#file-input {
    display: none;
  }
</style>