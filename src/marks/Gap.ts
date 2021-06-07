  
import {
  Command,
  Mark,
  markInputRule,
  markPasteRule,
  mergeAttributes,
} from '@tiptap/core'

export interface GapOptions {
  HTMLAttributes: Record<string, any>,
}

declare module '@tiptap/core' {
  interface Commands {
    gap: {
      /**
       * Set a gap mark
       */
      setGap: () => Command,
      /**
       * Toggle a gap mark
       */
      toggleGap: () => Command,
      /**
       * Unset a gap mark
       */
      unsetGap: () => Command,
    }
  }
}

export const pipeInputRegex = /(?:^|\s)((?:\|\|)((?:[^*]+))(?:\|\|))$/gm
export const pipePasteRegex = /(?:^|\s)((?:\|\|)((?:[^*]+))(?:\|\|))/gm

const name = 'gap'

export default Mark.create<GapOptions>({
  name: name,

  defaultOptions: {
    HTMLAttributes: {},
  },

  excludes: '_',

  parseHTML() {
    return [
      {
        tag: 'span',
        'data-type': name,
      },
    ]
  },

  renderHTML({ HTMLAttributes }) {
    return ['span', mergeAttributes({'data-type': name}, this.options.HTMLAttributes, HTMLAttributes), 0]
  },

  addCommands() {
    return {
      setGap: () => ({ commands }) => {
        return commands.setMark('gap')
      },
      toggleGap: () => ({ commands }) => {
        return commands.toggleMark('gap')
      },
      unsetGap: () => ({ commands }) => {
        return commands.unsetMark('gap')
      },
    }
  },

  addKeyboardShortcuts() {
    return {
      'Mod-g': () => this.editor.commands.toggleGap(),
    }
  },

  addInputRules() {
    return [
      markInputRule(pipeInputRegex, this.type),
    ]
  },

  addPasteRules() {
    return [
      markPasteRule(pipePasteRegex, this.type),
    ]
  },
})