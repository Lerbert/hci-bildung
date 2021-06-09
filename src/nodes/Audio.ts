import {
  Command,
  Node,
  mergeAttributes,
} from '@tiptap/core'
import { upload } from '../storage'

declare module '@tiptap/core' {
  interface Commands {
    audio: {
      setAudio: (source: string, mimetype: string) => Command,
    }
  }
}

export default Node.create({
  name: 'audio',

  content: '',
  group: 'block',
  marks: '_',
  atom: true,

  addAttributes() {
    return {
      source: {
        default: 'https://4m6.de/rick.ogg',
        renderHTML: _ => {},
      },
      mimetype: {
        default: 'audio/ogg',
        renderHTML: _ => {},
      },
    }
  },

  parseHTML() {
    return [
      {
        tag: 'audio',
        getAttrs: node => {
          const audio = node as HTMLAudioElement;
          const source = audio.children[0] as HTMLSourceElement;
          return {
            source: source.src,
            mimetype: source.type,
          };
        },
      },
    ]
  },

  renderHTML({ node }) {
    return ['audio', mergeAttributes({ 'controls': true }), ['source', { src: node.attrs.source, type: node.attrs.mimetype }]]
  },

  addCommands() {
    return {
      setAudio: (source, mimetype) => ({ commands }) => {
        return commands.insertContent({
          type: this.name,
          attrs: { source, mimetype, },
        })
      },
    }
  },

  addKeyboardShortcuts() {
    return {
      'Mod-m': () => { upload(this.editor.commands.setAudio); return true; },
    }
  },
})