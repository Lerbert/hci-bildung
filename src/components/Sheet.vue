<template>
  <div class="sheet">
    <div class="sheet__header">
      <button class="button is-success is-small" v-on:click="checkAll">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" width="24" height="24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" />
        </svg>
        &nbsp;
        Alle überprüfen
      </button>
    </div>
    <node class="sheet__content" :tiptapNode="editorJSON" :checkTrigger="checkTrigger"></node>
  </div>
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

<style lang="scss">
  .sheet {
    display: flex;
    flex-direction: column;
    max-height: 400px;
    color: #0D0D0D;
    background-color: #FFFFFF;
    border: 3px solid #0D0D0D;
    border-radius: 0.75rem;

    &__header {
      display: flex;
      align-items: center;
      flex: 0 0 auto;
      flex-wrap: wrap;
      padding: 0.25rem;
      padding-bottom: 0.1875rem;
      border-bottom: 3px solid #0D0D0D;
      & > {
        height: 1.75rem;
        padding: 0.25rem;
        margin-right: 0.25rem;
      }
    }

    &__content {
      padding: 0.25rem;
      flex: 1 1 auto;
      overflow-x: hidden;
      overflow-y: auto;
      -webkit-overflow-scrolling: touch;
    }
  }
</style>