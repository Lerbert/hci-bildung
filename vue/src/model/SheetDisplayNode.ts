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
      case "audio":
        return new Audio(tiptapNode);
      case "codeBlock":
        return new Codeblock(tiptapNode);
      case "heading":
        return new Heading(tiptapNode);
      case "multipleChoice":
        return new MultipleChoice(tiptapNode);
      case "multipleChoiceAnswer":
        return new MultipleChoiceAnswer(tiptapNode);
      case "text":
        return new Text(tiptapNode);
      default:
        return new Node(tiptapNode);
    }
  }
}

export class Audio extends Node {
  source: string;
  mimetype: string;

  constructor(tiptapNode: JSONContent) {
    super(tiptapNode);
    this.source = tiptapNode.attrs?.source ?? "";
    this.mimetype = tiptapNode.attrs?.mimetype ?? "";
  }
}

export class Codeblock extends Node {
  language: string;

  constructor(tiptapNode: JSONContent) {
    super(tiptapNode);
    this.language = tiptapNode.attrs?.language ?? "plain";
  }
}

export class Heading extends Node {
  level: number;

  constructor(tiptapNode: JSONContent) {
    super(tiptapNode);
    this.level = tiptapNode.attrs?.level ?? 1;
  }
}

export class MultipleChoice extends Node {
  declare content: MultipleChoiceAnswer[];

  constructor(tiptapNode: JSONContent) {
    super(tiptapNode);
    if (!this.content.every((node) => node instanceof MultipleChoiceAnswer)) {
      throw new Error(
        "MultipleChoice.content must only contain MultipleChoiceAnswer"
      );
    }
    this.content = this.content.map((n) => n as MultipleChoiceAnswer);
  }
}

export class MultipleChoiceAnswer extends Node {
  solution: boolean;
  answer: boolean;

  constructor(tiptapNode: JSONContent) {
    super(tiptapNode);
    this.solution = tiptapNode.attrs?.checked ?? false;
    this.answer = false;
  }
}

export class Text extends Node {
  text: string;

  constructor(tiptapNode: JSONContent) {
    super(tiptapNode);
    this.text = tiptapNode.text ?? "";
  }
}
