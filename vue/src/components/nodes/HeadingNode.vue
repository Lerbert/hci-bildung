<script>
import { defineComponent, h } from "vue";

export default defineComponent({
  props: {
    sheet: {
      type: Object,
      required: true,
    },
  },

  render() {
    if (this.sheet.content.length > 0) {
      return h("h" + this.sheet.level, {}, this.$slots.default());
    } else {
      return h("h" + this.sheet.level, {}, [this.$slots.default(), h("br")]);
    }
  },
});
</script>

<style></style>
