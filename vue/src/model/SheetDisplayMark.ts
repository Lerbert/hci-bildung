import { JSONContent } from "@tiptap/vue-3";

type JSONContentMark = {
  type: string;
  attrs?: Record<string, any>;
  [key: string]: any;
};

export class Mark {
  type?: string;

  constructor(tiptapMark: JSONContentMark) {
    this.type = tiptapMark.type;
  }

  public static fromTiptap(
    tiptapMark: JSONContentMark,
    parentNode: JSONContent
  ): Mark {
    switch (tiptapMark.type) {
      case "gap":
        return new Gap(tiptapMark, parentNode);
      case "latex":
        return new Latex(tiptapMark, parentNode);
      default:
        return new Mark(tiptapMark);
    }
  }
}

export class Gap extends Mark {
  solution: string;
  answer: string;

  constructor(tiptapMark: JSONContentMark, parentNode: JSONContent) {
    super(tiptapMark);
    this.solution = parentNode.text ?? "";
    this.answer = "";
  }
}

export class Latex extends Mark {
  source: string;

  constructor(tiptapMark: JSONContentMark, parentNode: JSONContent) {
    super(tiptapMark);
    this.source = parentNode.text ?? "";
  }
}
