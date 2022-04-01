import { JSONContent } from "@tiptap/vue-3";

import { Mark } from "./SheetDisplayMark";

export class Node {
  type?: string;
  content: Node[];
  marks: Mark[];

  constructor(tiptapNode: JSONContent) {
    this.type = tiptapNode.type;
    this.content =
      tiptapNode.content?.map((node) => Node.fromTiptap(node)) ?? [];
    this.marks =
      tiptapNode.marks?.map((mark) => Mark.fromTiptap(mark, tiptapNode)) ?? [];
  }

  public static fromTiptap(tiptapNode: JSONContent): Node {
    switch (tiptapNode.type) {
      case "text":
        return new Text(tiptapNode);
      case "multipleChoiceAnswer":
        return new MultipleChoiceAnswer(tiptapNode);
      default:
        return new Node(tiptapNode);
    }
  }
}

export class Text extends Node {
  text: string;

  constructor(tiptapNode: JSONContent) {
    super(tiptapNode);
    this.text = tiptapNode.text ?? "";
  }
}

export class MultipleChoiceAnswer extends Node {
  solution: boolean;
  checked: boolean;

  constructor(tiptapNode: JSONContent) {
    super(tiptapNode);
    this.solution = tiptapNode.attrs?.checked ?? false;
    this.checked = false;
  }
}
