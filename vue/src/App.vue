<template>
  <div class="container is-fluid content" id="app">
    <edit-view v-if="edit" :docJSON="docJSON" :docTitle="docTitle"></edit-view>
    <student-view v-else :docJSON="docJSON" :docTitle="docTitle"></student-view>
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
      docJSON: {
        type: Object,
        default: () => ({type: 'doc', content: [{type: 'paragraph'}]}),
      },
      docTitle: {
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