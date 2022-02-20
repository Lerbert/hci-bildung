<template>
  <span ref="latex"></span>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Katex from "katex";

export default defineComponent({
  props: {
    tiptapNode: {
      type: Object,
      required: true,
    },
  },

  data() {
    return {};
  },

  computed: {
    latexSource(): string {
      return this.tiptapNode.text;
    },
  },

  mounted() {
    this.renderLatex();
  },

  methods: {
    renderLatex() {
      Katex.render(this.latexSource, this.$refs.latex as HTMLElement, {
        throwOnError: false,
      });
    },
  },

  watch: {
    latexSource: function () {
      this.renderLatex();
    },
  },
});
</script>

<style lang="css" scoped>
@import "../../assets/katex/katex.min.css";
</style>
