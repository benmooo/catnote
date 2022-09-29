import { OutputData } from "@editorjs/editorjs";

export const DEFAULT_EDITOR_DATA: OutputData = {
  time: Date.now(),
  blocks: [
    {
      id: '7niHoxlUUr',
      type: 'header',
      data: {
        text: 'Editor.js',
        level: 1,
      },
    },
    {
      id: 'luwu36WmLM',
      type: 'paragraph',
      data: {
        text: 'Hey. Meet the new Editor. On this page you can see it in action — try to edit this text.',
      },
    },
    {
      id: 'ufBofGCbo8',
      type: 'header',
      data: {
        text: 'Key features',
        level: 3,
      },
    },
    {
      id: 'QgXG4Y6nAR',
      type: 'list',
      data: {
        style: 'unordered',
        items: [
          'It is a block-styled editor',
          'It returns clean data output in JSON',
          'Designed to be extendable and pluggable with a simple API',
        ],
      },
    },
    {
      id: 'tac95XDRjC',
      type: 'header',
      data: {
        text: 'What does it mean «block-styled editor»',
        level: 3,
      },
    },
    {
      id: 'BXUFoAnrDe',
      type: 'paragraph',
      data: {
        text: 'Workspace in classic editors is made of a single contenteditable element, used to create different HTML markups. Editor.js <mark class="cdx-marker">workspace consists of separate Blocks: paragraphs, headings, images, lists, quotes, etc</mark>. Each of them is an independent contenteditable element (or more complex structure) provided by Plugin and united by Editor\'s Core.',
      },
    },
    {
      id: 'V_q6YY6g4U',
      type: 'paragraph',
      data: {
        text: 'There are dozens of <a href="https://github.com/editor-js">ready-to-use Blocks</a> and the <a href="https://editorjs.io/creating-a-block-tool">simple API</a> for creation any Block you need. For example, you can implement Blocks for Tweets, Instagram posts, surveys and polls, CTA-buttons and even games.',
      },
    },
    {
      id: 'dAknDEPKi4',
      type: 'header',
      data: {
        text: 'What does it mean clean data output',
        level: 3,
      },
    },
    {
      id: 'G-NTJCJsEx',
      type: 'paragraph',
      data: {
        text: 'Classic WYSIWYG-editors produce raw HTML-markup with both content data and content appearance. On the contrary, Editor.js outputs JSON object with data of each Block. You can see an example below',
      },
    },
    {
      id: 'Z_cCMV9cNN',
      type: 'paragraph',
      data: {
        text: 'Given data can be used as you want: render with HTML for <code class="inline-code">Web clients</code>, render natively for <code class="inline-code">mobile apps</code>, create markup for <code class="inline-code">Facebook Instant Articles</code> or <code class="inline-code">Google AMP</code>, generate an <code class="inline-code">audio version</code> and so on.',
      },
    },
    {
      id: 'LIwYP0hYGt',
      type: 'paragraph',
      data: {
        text: 'Clean data is useful to sanitize, validate and process on the backend.',
      },
    },
    {
      id: 'lmRnEXZPMg',
      type: 'delimiter',
      data: {},
    },
    {
      id: '3n3Pirgoq5',
      type: 'paragraph',
      data: {
        text: "We have been working on this project more than three years. Several large media projects help us to test and debug the Editor, to make it's core more stable. At the same time we significantly improved the API. Now, it can be used to create any plugin for any task. Hope you enjoy. 😏",
      },
    },
    {
      id: 'ylbJ81xef8',
      type: 'image',
      data: {
        file: {
          url: 'https://codex.so/public/app/img/external/codex2x.png',
        },
        caption: '',
        withBorder: false,
        stretched: false,
        withBackground: false,
      },
    },
  ],
  version: '2.24.3',
}
