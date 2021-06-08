<template>
  <node :tiptapNode="editorJSON" :checkTrigger="checkTrigger"></node>
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

  data() {
    return {
      checkTrigger: false,
    }
  },

  methods: {
    checkAll() {
      this.checkTrigger = !this.checkTrigger;
    }
  },
});
</script>

<style>

</style>