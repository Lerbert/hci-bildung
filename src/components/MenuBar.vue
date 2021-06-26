<template>
  <div>
    <template v-for="(item, index) in items" :key="index">
      <div class="divider" v-if="item.type === 'divider'" />
      <menu-item v-else v-bind="item" />
    </template>
  </div>
</template>

<script>
import MenuItem from './MenuItem.vue'
import { upload } from '../storage'

export default {
  components: {
    MenuItem,
  },

  props: {
    editor: {
      type: Object,
      required: true,
    },
  },

  data() {
    return {
      items: [
        {
          icon: 'bold',
          title: 'Fett (Strg + B)',
          action: () => this.editor.chain().focus().toggleBold().run(),
          isActive: () => this.editor.isActive('bold'),
        },
        {
          icon: 'italic',
          title: 'Kursiv (Strg + I)',
          action: () => this.editor.chain().focus().toggleItalic().run(),
          isActive: () => this.editor.isActive('italic'),
        },
        {
          icon: 'strikethrough',
          title: 'Durchstreichen (Strg + Shift + X)',
          action: () => this.editor.chain().focus().toggleStrike().run(),
          isActive: () => this.editor.isActive('strike'),
        },
        {
          type: 'divider',
        },
        {
          icon: 'h-1',
          title: 'Große Überschrift (Strg + Alt + 1)',
          action: () => this.editor.chain().focus().toggleHeading({ level: 1 }).run(),
          isActive: () => this.editor.isActive('heading', { level: 1 }),
        },
        {
          icon: 'h-2',
          title: 'Kleine Überschrift (Strg + Alt + 2)',
          action: () => this.editor.chain().focus().toggleHeading({ level: 2 }).run(),
          isActive: () => this.editor.isActive('heading', { level: 2 }),
        },
        {
          icon: 'text',
          title: 'Absatz (Strg + Alt + 0)',
          action: () => this.editor.chain().focus().setParagraph().run(),
          isActive: () => this.editor.isActive('paragraph'),
        },
        {
          icon: 'list-unordered',
          title: 'Auflistung einfügen (Strg + Shift + 8)',
          action: () => this.editor.chain().focus().toggleBulletList().run(),
          isActive: () => this.editor.isActive('bulletList'),
        },
        {
          icon: 'list-ordered',
          title: 'Aufzählung einfügen (Strg + Shift + 7)',
          action: () => this.editor.chain().focus().toggleOrderedList().run(),
          isActive: () => this.editor.isActive('orderedList'),
        },
        {
          type: 'divider',
        },
        {
          icon: 'text-wrap',
          title: 'Zeilenumbruch erzwingen (Shift + Enter)',
          action: () => this.editor.chain().focus().setHardBreak().run(),
        },
        {
          icon: 'format-clear',
          title: 'Formatierung entfernen',
          action: () => this.editor.chain()
            .focus()
            .clearNodes()
            .unsetAllMarks()
            .run(),
        },
        {
          type: 'divider',
        },
        {
          icon: 'arrow-go-back-line',
          title: 'Änderung zurücknehmen (Strg + Z)',
          action: () => this.editor.chain().focus().undo().run(),
        },
        {
          icon: 'arrow-go-forward-line',
          title: 'Änderung wiederholen (Strg + Shift + Z)',
          action: () => this.editor.chain().focus().redo().run(),
        },
        {
          type: 'divider',
        },
        {
          icon: 'space',
          title: 'Auswahl in Lücke umwandeln (Strg + G)',
          action: () => this.editor.chain().focus().toggleGap().run(),
          isActive: () => this.editor.isActive('gap'),
        },
        {
          icon: 'volume-up-line',
          title: 'Audioelement hinzufügen (Strg + M)',
          action: () => upload((url, type) => this.editor.chain().focus().setAudio(url, type).run()),
        },
      ],
    }
  },
}
</script>

<style lang="scss" scoped>
.divider {
  width: 2px;
  height: 1.25rem;
  background-color: rgba(#000000, 0.1);
  margin-left: 0.5rem;
  margin-right: 0.75rem;
}
</style>