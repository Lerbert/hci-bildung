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
import { Editor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'

import Gap from '../marks/Gap.ts'
import Audio from '../nodes/Audio.ts'
import MenuBar from './MenuBar.vue'
import SaveStatus from './SaveStatus.vue'

export default {
  components: {
    EditorContent,
    MenuBar,
    SaveStatus,
  },

  props: {
    initialContent: {
      type: Object,
      default: () => ({type: 'doc', content: [{type: 'paragraph'}]}),
    },
    saveStatus: {
      required: true,
    }
  },

  data() {
    return {
      editor: null,
      content: this.initialContent,
    }
  },

  mounted() {
    this.editor = new Editor({
      content: this.content,
      extensions: [
        StarterKit,
        Gap,
        Audio,
      ],
      onUpdate: () => {
        this.$emit('update:content', this.editor.getJSON())
      },
    })
  },

  beforeUnmount() {
    this.editor.destroy()
  },
}
</script>

<style lang="scss" scoped>
.editor {
  display: flex;
  flex-direction: column;
  max-height: 400px;
  color: #0D0D0D;
  background-color: #FFFFFF;
  border: 3px solid #0D0D0D;
  border-radius: 0.75rem;

  &__header {
    display: flex;
    align-items: center;
    flex: 0 0 auto;
    flex-wrap: wrap;
    padding: 0.25rem;
    border-bottom: 3px solid #0D0D0D;
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
    border-top: 3px solid #0D0D0D;
    font-size: 12px;
    font-weight: 600;
    color: #0D0D0D;
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
    background: #0D0D0D;
    color: #FFF;
    font-family: 'JetBrainsMono', monospace;
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
    color: #FFFFFF;
    background-color: #3D3D3D;
  }
}
</style>