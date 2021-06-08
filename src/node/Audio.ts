import {
  Command,
  Node,
  mergeAttributes,
} from '@tiptap/core'

import AudioPopup from './AudioPopup.vue'

export interface AudioOptions {
  source: string,
  mimetype: string,
}

declare module '@tiptap/core' {
  interface Commands {
    audio: {
      addAudio: () => Command,
      removeAudio: () => Command,
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

  parseHTML() {
    return [
      {
        tag: 'audio',
      },
    ]
  },

  renderHTML({ HTMLAttributes }) {
    return ['audio', mergeAttributes(HTMLAttributes, {'controls': true}), ['source', {'type': this.options.mimetype, 'src': this.options.source}, 0]]
  },

  addCommands() {
    return {
      addAudio: () => ({ commands }) => {
        console.log("Hello");
        AudioPopup.watch?.showModal(commands);
        console.log(AudioPopup.methods?.getShow());
        return commands.insertContent({
          type: this.name,
        })
      },
    }
  },

  addKeyboardShortcuts() {
    return {
      'Mod-m': () => this.editor.commands.addAudio(),
    }
  },
})