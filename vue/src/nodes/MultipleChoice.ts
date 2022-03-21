import { Node, mergeAttributes, getNodeType } from "@tiptap/core";
import { setBlockType } from "prosemirror-commands";

export interface ChoiceOptions {
  answerTypeName: string;
  HTMLAttributes: Record<string, any>;
}

declare module "@tiptap/core" {
  interface Commands<ReturnType> {
    choice: {
      /**
       * Toggle a multiple choice input
       */
      toggleMultipleChoice: () => ReturnType;
    };
  }
}

export default Node.create<ChoiceOptions>({
  name: "multipleChoice",

  addOptions() {
    return {
      answerTypeName: "multipleChoiceAnswer",
      HTMLAttributes: {},
    };
  },

  group: "block list",

  content() {
    return `${this.options.answerTypeName}+`;
  },

  parseHTML() {
    return [
      {
        tag: `ul[data-type="${this.name}"]`,
        priority: 51,
      },
    ];
  },

  renderHTML({ HTMLAttributes }) {
    return [
      "ul",
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
      toggleMultipleChoice:
        () =>
        ({ commands }) => {
          return commands.toggleList(this.name, this.options.answerTypeName);
        },
    };
  },

  addKeyboardShortcuts() {
    return {
      "Mod-M": () => this.editor.commands.toggleMultipleChoice(),
    };
  },
});
