<template>
  <figure>
    <div class="block is-flex is-justify-content-center">
      <canvas ref="canvas"></canvas>
    </div>
  </figure>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import QRCode from "qrcode";

export default defineComponent({
  props: {
    value: {
      type: String,
      required: true,
    },
  },

  methods: {
    refreshQR() {
      QRCode.toCanvas(this.$refs.canvas, this.value, { width: 256 }, (e) => {
        if (e) {
          console.log(e);
        }
      });
    },
  },

  mounted() {
    this.refreshQR();
  },

  watch: {
    value: function () {
      this.refreshQR();
    },
  },
});
</script>

<style lang="scss" scoped></style>
