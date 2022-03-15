<template>
  <div class="dropdown" :class="{ 'is-active': more }">
    <div class="dropdown-trigger">
      <button
        class="button"
        aria-haspopup="true"
        :aria-controls="dropdownId"
        @click="toggle"
      >
        <span class="icon is-small">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width="2"
            width="24"
            height="24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"
            />
          </svg>
        </span>
      </button>
    </div>
    <div class="dropdown-menu" :id="dropdownId" role="menu">
      <div class="dropdown-content">
        <slot></slot>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "MoreButton",

  props: {
    dropdownId: {
      type: String,
      default: "dropdown-more",
    },
  },

  data() {
    return {
      more: false,
    };
  },

  mounted() {
    document.addEventListener("click", this.clickHandler);
  },

  beforeUnmount() {
    document.removeEventListener("click", this.clickHandler);
  },

  methods: {
    toggle() {
      this.more = !this.more;
    },

    clickHandler(event: Event) {
      const { target } = event;
      const { $el } = this;
      if (!$el.contains(target)) {
        this.more = false;
      }
    },
  },
});
</script>

<style lang="scss" scoped></style>
