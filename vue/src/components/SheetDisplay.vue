<template>
  <div class="sheet">
    <div class="sheet__header">
      <button class="button is-success is-small" v-on:click="checkAll">
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
            d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"
          />
        </svg>
        &nbsp; Alle überprüfen
      </button>
    </div>
    <sheet-node
      class="sheet__content"
      :tiptapNode="editorJSON"
      :checkTrigger="checkTrigger"
      @grantPoints="grantPoints"
    ></sheet-node>
    <div class="sheet__footer">
      <point-status
        :achievedPoints="achievedPoints"
        :totalPoints="totalPoints"
      ></point-status>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import SheetNode from "./nodes/SheetNode.vue";
import PointStatus from "./PointStatus.vue";

export default defineComponent({
  components: {
    SheetNode,
    PointStatus,
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
      achievedPoints: 0,
      totalPoints: 0,
    };
  },

  methods: {
    checkAll() {
      this.achievedPoints = 0;
      this.totalPoints = 0;
      this.checkTrigger = !this.checkTrigger;
    },
    grantPoints(event: { achievedPoints: number; totalPoints: number }) {
      this.achievedPoints += event.achievedPoints;
      this.totalPoints += event.totalPoints;
    },
  },
});
</script>

<style lang="scss">
.sheet {
  display: flex;
  flex-direction: column;
  max-height: 400px;
  color: #0d0d0d;
  background-color: #ffffff;
  border: 3px solid #0d0d0d;
  border-radius: 0.75rem;

  &__header {
    display: flex;
    align-items: center;
    flex: 0 0 auto;
    flex-wrap: wrap;
    padding: 0.25rem;
    padding-bottom: 0.1875rem;
    border-bottom: 3px solid #0d0d0d;
    & > {
      height: 1.75rem;
      padding: 0.25rem;
      margin-right: 0.25rem;
    }
  }

  &__content {
    padding: 0.5rem;
    flex: 1 1 auto;
    overflow-x: hidden;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
  }

  &__footer {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: flex-end;
    flex-wrap: wrap;
    white-space: nowrap;
    border-top: 3px solid #0d0d0d;
    font-size: 12px;
    font-weight: 600;
    color: #0d0d0d;
    white-space: nowrap;
    padding: 0.25rem 0.75rem;
  }
}

.sheet__content {
  > * + * {
    margin-top: 0.75em;
  }

  ul,
  ol {
    padding: 0 1rem;
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    line-height: 1.1;
  }

  code {
    background-color: rgba(#616161, 0.1);
    color: #616161;
  }

  pre {
    background: #0d0d0d;
    color: #fff;
    font-family: "JetBrainsMono", monospace;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    white-space: pre-wrap;

    code {
      color: inherit;
      padding: 0;
      background: none;
      font-size: 0.8rem;
    }
  }

  img {
    max-width: 100%;
    height: auto;
  }

  span[data-type="gap"] {
    color: #ffffff;
    background-color: #3d3d3d;
  }
}
</style>