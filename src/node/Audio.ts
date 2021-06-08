import {
  Command,
  Node,
  mergeAttributes,
} from '@tiptap/core'

declare module '@tiptap/core' {
  interface Commands {
    audio: {
      setAudio: (source: string, mimetype: string) => Command,
      setAudioUpload: () => Command,
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
      'Mod-m': () => this.editor.commands.setAudio('https://esclear.de/public/.abifeier/abi_intro_v2.ogg', 'audio/ogg'),
    }
  },
})