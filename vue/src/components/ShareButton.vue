<template>
  <div>
    <button class="button is-info js-modal-trigger" data-target="share-modal">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        width="24"
        height="24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"
        />
      </svg>
      &nbsp; Mit Schüler*innen teilen
    </button>

    <div id="share-modal" class="modal">
      <div class="modal-background"></div>
      <div class="modal-card">
        <header class="modal-card-head">
          <span class="modal-card-title">Mit Schüler*innen teilen</span>
          <button class="delete" aria-label="close"></button>
        </header>
        <section class="modal-card-body">
          <q-r-code :value="link"></q-r-code>
          <div class="field has-addons">
            <div class="control is-expanded">
              <input class="input" type="text" v-model="link" readonly />
            </div>
            <div class="control">
              <button class="button is-success" v-on:click="copy">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-6 w-6"
                  fill="none"
                  viewBox="0 0 24 24"
                  width="24"
                  height="24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3"
                  />
                </svg>
                &nbsp;
                {{ copyText }}
              </button>
            </div>
          </div>
        </section>
        <footer class="modal-card-foot"></footer>
      </div>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";

import QRCode from "./QRCode.vue";

export default defineComponent({
  components: {
    QRCode,
  },

  props: {
    docId: {
      type: String,
      required: true,
    },
  },

  data() {
    return {
      copyText: "Link kopieren",
      copyTimeout: undefined,
    };
  },

  computed: {
    link() {
      return window.location.origin + "/sheets/" + this.docId;
    },
  },

  methods: {
    copy() {
      if (this.copyTimeout) {
        clearTimeout(this.copyTimeout);
      }

      navigator.clipboard.writeText(this.link);

      this.copyText = "Link kopiert!";
      this.copyTimeout = setTimeout(
        () => (this.copyText = "Link kopieren"),
        3000
      );
    },
  },
});
</script>

<style lang="scss" scoped>
#share-modal {
  .modal-card {
    width: 50%;
  }
}
</style>
