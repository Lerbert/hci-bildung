<script>
import { defineComponent, h } from 'vue';

export default defineComponent({
  props: {
    tiptapNode: {
      type: Object,
      required: true,
    }
  },

  render() {
    if (this.tiptapNode.content) {
      return h('h' + this.tiptapNode.attrs.level, {}, this.$slots.default());
    } else {
      return h('h' + this.tiptapNode.attrs.level, {}, [this.$slots.default(), h('br')]);
    }
  }
});
</script>

<style>

</style>