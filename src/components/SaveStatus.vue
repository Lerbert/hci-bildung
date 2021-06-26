<template>
  <div class="spinner_container">
    <div v-if="!saved" class="spinner"></div>
    <svg v-if="saved" color="#48c774" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" height=18px stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
    </svg>
    {{ text }}
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';


export default defineComponent({
  data() {
    return {
      saved: false,
    }
  },

  computed: {
    text(): string {
      return this.saved ? "Gespeichert" : "Wird gespeichert";
    }
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
  border:  2px solid #fff;
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