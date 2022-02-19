import {
  Mark,
  markInputRule,
  markPasteRule,
  mergeAttributes,
} from "@tiptap/core";

export interface GapOptions {
  HTMLAttributes: Record<string, any>;
}

declare module "@tiptap/core" {
  interface Commands<ReturnType> {
    gap: {
      /**
       * Set a gap mark
       */
      setGap: () => ReturnType;
      /**
       * Toggle a gap mark
       */
      toggleGap: () => ReturnType;
      /**
       * Unset a gap mark
       */
      unsetGap: () => ReturnType;
    };
  }
}

export const pipeInputRegex = /(?:^|\s)((?:\|\|)((?:[^*]+))(?:\|\|))$/gm;
export const pipePasteRegex = /(?:^|\s)((?:\|\|)((?:[^*]+))(?:\|\|))/gm;

const name = "gap";

export default Mark.create<GapOptions>({
  name: name,

  addOptions() {
    return {
      HTMLAttributes: {},
    };
  },

  excludes: "_",

  parseHTML() {
    return [
      {
        tag: "span",
        getAttrs: (element) => {
          const dataType = (element as HTMLElement).getAttribute("data-type");

          if (!dataType || dataType != name) {
            return false;
          }

          return {};
        },
      },
    ];
  },

  renderHTML({ HTMLAttributes }) {
    return [
      "span",
      mergeAttributes(
        { "data-type": name },
        this.options.HTMLAttributes,
        HTMLAttributes
      ),
      0,
    ];
  },

  addCommands() {
    return {
      setGap:
        () =>
        ({ commands }) => {
          return commands.setMark(this.name);
        },
      toggleGap:
        () =>
        ({ commands }) => {
          return commands.toggleMark(this.name);
        },
      unsetGap:
        () =>
        ({ commands }) => {
          return commands.unsetMark(this.name);
        },
    };
  },

  addKeyboardShortcuts() {
    return {
      "Mod-g": () => this.editor.commands.toggleGap(),
    };
  },

  addInputRules() {
    return [
      markInputRule({
        find: pipeInputRegex,
        type: this.type,
      }),
    ];
  },

  addPasteRules() {
    return [
      markPasteRule({
        find: pipePasteRegex,
        type: this.type,
      }),
    ];
  },
});
