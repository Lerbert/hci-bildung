import {
  Mark,
  markInputRule,
  markPasteRule,
  mergeAttributes,
} from "@tiptap/core";

export interface GapOptions {
  HTMLAttributes: Record<string, unknown>;
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

export const pipeInputRegex = /(?:^|\s)((?:\|\|)((?:[^|]+))(?:\|\|))$/;
export const pipePasteRegex = /(?:^|\s)((?:\|\|)((?:[^|]+))(?:\|\|))/g;

export default Mark.create<GapOptions>({
  name: "gap",

  addOptions() {
    return {
      HTMLAttributes: {},
    };
  },

  excludes: "_",

  parseHTML() {
    return [
      {
        tag: `span[data-type="${this.name}"]`,
      },
    ];
  },

  renderHTML({ HTMLAttributes }) {
    return [
      "span",
      mergeAttributes(
        { "data-type": this.name },
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
