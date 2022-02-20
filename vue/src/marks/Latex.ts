import {
  Mark,
  markInputRule,
  markPasteRule,
  mergeAttributes,
} from "@tiptap/core";

export interface LatexOptions {
  HTMLAttributes: Record<string, any>;
}

declare module "@tiptap/core" {
  interface Commands<ReturnType> {
    latex: {
      /**
       * Set a latex mark
       */
      setLatex: () => ReturnType;
      /**
       * Toggle a latex mark
       */
      toggleLatex: () => ReturnType;
      /**
       * Unset a latex mark
       */
      unsetLatex: () => ReturnType;
    };
  }
}

export const dollarInputRegex = /(?:^|\s)((?:\$\$)((?:[^$$]+))(?:\$\$))$/;
export const dollarPasteRegex = /(?:^|\s)((?:\$\$)((?:[^$$]+))(?:\$\$))/g;

const name = "latex";

export default Mark.create<LatexOptions>({
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
      setLatex:
        () =>
        ({ commands }) => {
          return commands.setMark(this.name);
        },
      toggleLatex:
        () =>
        ({ commands }) => {
          return commands.toggleMark(this.name);
        },
      unsetLatex:
        () =>
        ({ commands }) => {
          return commands.unsetMark(this.name);
        },
    };
  },

  addKeyboardShortcuts() {
    return {
      "Alt-=": () => this.editor.commands.toggleLatex(),
    };
  },

  addInputRules() {
    return [
      markInputRule({
        find: dollarInputRegex,
        type: this.type,
      }),
    ];
  },

  addPasteRules() {
    return [
      markPasteRule({
        find: dollarPasteRegex,
        type: this.type,
      }),
    ];
  },
});
