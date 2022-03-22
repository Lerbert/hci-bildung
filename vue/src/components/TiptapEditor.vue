<template>
  <div class="editor" v-if="editor">
    <menu-bar class="editor__header" :editor="editor" />
    <editor-content class="editor__content" :editor="editor" />
    <div class="editor__footer">
      <save-status :saveStatus="saveStatus"></save-status>
    </div>
  </div>
</template>

<script>
import { Editor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";

import Audio from "../nodes/Audio.ts";
import Gap from "../marks/Gap.ts";
import Latex from "../marks/Latex.ts";
import MenuBar from "./MenuBar.vue";
import MultipleChoiceAnswer from "../nodes/MultipleChoiceAnswer.ts";
import MultipleChoice from "../nodes/MultipleChoice.ts";
import SaveStatus from "./SaveStatus.vue";

export default {
  components: {
    EditorContent,
    MenuBar,
    SaveStatus,
  },

  props: {
    initialContent: {
      type: Object,
      default: () => ({ type: "doc", content: [{ type: "paragraph" }] }),
    },
    saveStatus: {
      required: true,
    },
  },

  data() {
    return {
      editor: null,
      content: this.initialContent,
    };
  },

  mounted() {
    this.editor = new Editor({
      content: this.content,
      extensions: [
        StarterKit,
        Audio,
        Gap,
        Latex,
        MultipleChoiceAnswer,
        MultipleChoice,
      ],
      onUpdate: () => {
        this.$emit("update:content", this.editor.getJSON());
      },
    });
  },

  beforeUnmount() {
    this.editor.destroy();
  },
};
</script>

<style lang="scss" scoped>
.editor {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  max-height: 600px;
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
    border-bottom: 3px solid #0d0d0d;
  }

  &__content {
    padding: 0.5rem;
    flex: 1 1 auto;
    overflow-x: hidden;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    background-color: hsla(0, 0%, 90%, 90%);
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
</style>

<style lang="scss">
/* Basic editor styles */
.ProseMirror {
  &:focus {
    outline: none;
  }
  > * + * {
    margin-top: 0.75em;
  }

  ul,
  ol {
    padding: 0 1rem;
  }

  ul[data-type="multipleChoice"] {
    list-style: none;
    padding: 0;

    li {
      align-items: center;
      display: flex;

      > label {
        flex: 0 0 auto;
        margin-right: 0.5rem;
        user-select: none;
      }

      > div {
        flex: 1 1 auto;
      }
    }
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

  span[data-type="latex"] {
    font-family: "Courier New", "Lucida Console", monospace;
  }
}
</style>
