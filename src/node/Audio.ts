import {
  Command,
  Node,
  mergeAttributes,
} from '@tiptap/core'
import { TextSelection } from 'prosemirror-state'

export interface AudioOptions {
  source: string,
  mimetype: string,
}

declare module '@tiptap/core' {
  interface Commands {
    audio: {
      setAudio: () => Command,
    }
  }
}

export default Node.create<AudioOptions>({
  name: 'audio',

  defaultOptions: {
    source: 'https://esclear.de/public/.abifeier/abi_intro_v2.ogg',
    mimetype: 'audio/ogg',
  },

  content: '',
  marks: '_',
  atom: true,

  group: 'block',

  parseHTML() {
    return [
      {
        tag: 'audio',
      },
    ]
  },

  renderHTML({ HTMLAttributes }) {
    return ['audio', mergeAttributes(HTMLAttributes, { 'controls': true }), ['source', { 'type': this.options.mimetype, 'src': this.options.source }]]
  },

  addCommands() {
    return {
      setAudio: () => ({ chain }) => {
        return chain()
          .insertContent({ type: this.name })
          .command(({ tr, dispatch }) => {
            if (dispatch) {
              const { parent, pos } = tr.selection.$from
              const posAfter = pos + 1
              const nodeAfter = tr.doc.nodeAt(posAfter)

              // end of document
              if (!nodeAfter) {
                const node = parent.type.contentMatch.defaultType?.create()

                if (node) {
                  tr.insert(posAfter, node)
                  tr.setSelection(TextSelection.create(tr.doc, posAfter))
                }
              }

              tr.scrollIntoView()
            }

            return true
          })
          .run()
      }
    }
  },

  addKeyboardShortcuts() {
    return {
      'Mod-m': () => this.editor.commands.setAudio(),
    }
  },
})