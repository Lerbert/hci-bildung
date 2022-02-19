import { Node, mergeAttributes } from "@tiptap/core";
import { TextSelection } from "prosemirror-state";
import { upload } from "../storage";

declare module "@tiptap/core" {
  interface Commands<ReturnType> {
    audio: {
      setAudio: (source: string, mimetype: string) => ReturnType;
    };
  }
}

export default Node.create({
  name: "audio",

  content: "",
  group: "block",
  marks: "_",
  atom: true,

  addAttributes() {
    return {
      source: {
        default: "https://4m6.de/rick.ogg",
      },
      mimetype: {
        default: "audio/ogg",
      },
    };
  },

  parseHTML() {
    return [
      {
        tag: "audio",
        getAttrs: (node) => {
          const audio = node as HTMLAudioElement;
          const source = audio.children[0] as HTMLSourceElement;
          return {
            source: source.src,
            mimetype: source.type,
          };
        },
      },
    ];
  },

  renderHTML({ node }) {
    return [
      "audio",
      mergeAttributes({ controls: true }),
      ["source", { src: node.attrs.source, type: node.attrs.mimetype }],
    ];
  },

  addCommands() {
    return {
      setAudio:
        (source, mimetype) =>
        ({ chain }) => {
          return chain()
            .insertContent({
              type: this.name,
              attrs: { source, mimetype },
            })
            .command(({ tr, dispatch }) => {
              if (dispatch) {
                const { parent, pos } = tr.selection.$from;
                const posAfter = pos + 1;
                const nodeAfter = tr.doc.nodeAt(posAfter);

                // end of document
                if (!nodeAfter) {
                  const node = parent.type.contentMatch.defaultType?.create();

                  if (node) {
                    tr.insert(posAfter, node);
                    tr.setSelection(TextSelection.create(tr.doc, posAfter));
                  }
                }

                tr.scrollIntoView();
              }

              return true;
            })
            .run();
        },
    };
  },

  addKeyboardShortcuts() {
    return {
      "Mod-m": () => {
        upload(this.editor.commands.setAudio);
        return true;
      },
    };
  },
});
