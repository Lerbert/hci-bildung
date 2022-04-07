<template>
  <div class="spinner_container">
    <div v-if="saveStatus === SaveStatus.SAVING" class="spinner"></div>
    <svg
      v-if="saveStatus === SaveStatus.SAVED"
      color="#48c774"
      xmlns="http://www.w3.org/2000/svg"
      class="h-6 w-6"
      fill="none"
      viewBox="0 0 24 24"
      height="18px"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M5 13l4 4L19 7"
      />
    </svg>
    <svg
      v-if="saveStatus === SaveStatus.FAILED"
      color="#e31010"
      xmlns="http://www.w3.org/2000/svg"
      class="h-6 w-6"
      fill="none"
      viewBox="0 0 24 24"
      height="18px"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M6 18L18 6M6 6l12 12"
      />
    </svg>
    {{ text }}
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { SaveStatus } from "../enums";

export default defineComponent({
  props: {
    saveStatus: {
      required: true,
    },
  },

  data() {
    return {
      SaveStatus,
    };
  },

  computed: {
    text(): string {
      switch (this.saveStatus) {
        case SaveStatus.WAITING:
          return "Dokument wird bearbeitet";
        case SaveStatus.SAVING:
          return "Dokument wird gespeichert";
        case SaveStatus.SAVED:
          return "Dokument gespeichert";
        case SaveStatus.FAILED:
          return "Speichern fehlgeschlagen";
        case SaveStatus.DISABLED:
          return "Speichern deaktiviert";
        default:
          return "";
      }
    },
  },
});
</script>

<style lang="scss" scoped>
.spinner_container {
  display: flex;
  align-items: space-between;
  flex: 0 0 auto;
  flex-wrap: wrap;
}

// Taken from bulma code
.spinner {
  color: transparent !important;
  pointer-events: none;
  position: relative;
  width: 1.5em;
}

.spinner:after {
  animation: spinAround 500ms infinite linear;
  border: 2px solid #fff;
  border-radius: 50%;
  border-top-color: #48c77480;
  border-right-color: #48c774;
  content: "";
  display: block;
  width: 1em;
  height: 1em;
  position: relative;
  position: absolute;
  left: calc(50% - (1em / 2));
  top: calc(50% - (1em / 2));
  position: absolute !important;
}
@keyframes spinAround {
  from {
    -webkit-transform: rotate(0deg);
    transform: rotate(0deg);
  }
  to {
    -webkit-transform: rotate(359deg);
    transform: rotate(359deg);
  }
}
</style>
