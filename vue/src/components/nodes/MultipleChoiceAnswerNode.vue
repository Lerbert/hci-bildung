<template>
  <li>
    <label>
      <input type="checkbox" v-model="ticked" />
      <div><slot /></div>
    </label>
  </li>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    checkTrigger: {
      type: Boolean,
      required: true,
    },
    tiptapNode: {
      type: Object,
      required: true,
    },
  },

  emits: {
    answerCorrect(payload: { correct: boolean }) {
      return true;
    },
  },

  data() {
    return {
      ticked: false,
      checked: false,
      correct: false,
    };
  },

  computed: {
    solution(): boolean {
      return this.tiptapNode.attrs.checked;
    },
  },

  methods: {
    check() {
      this.checked = true;
      this.correct = this.ticked === this.solution;
      this.$emit("answerCorrect", {
        correct: this.correct,
      });
      console.log("emitted");
    },
  },

  watch: {
    checkTrigger() {
      this.check();
    },
  },
});
</script>

<style lang="scss" scoped>
label {
  align-items: center;
  display: flex;

  > input {
    flex: 0 0 auto;
    margin-right: 0.5rem;
  }

  > div {
    flex: 1 1 auto;
    min-width: 0%;
  }
}
</style>
