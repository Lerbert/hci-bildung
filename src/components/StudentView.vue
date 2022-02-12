<template>
  <h1>{{ docTitle }}</h1>
  <sheet :editorJSON="docJSON"></sheet>
</template>

<script>
import { defineComponent } from 'vue';

import Sheet from './Sheet.vue';

export default defineComponent({
  name: 'EditView',
  components: {
    Sheet,
  },

  props: {
    docJSON: {
      type: Object,
      required: true,
    },
    docTitle: {
      type: String,
      required: true,
    }
  },
});

</script>

<style>

</style>