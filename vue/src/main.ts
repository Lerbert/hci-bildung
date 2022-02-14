import { createApp } from 'vue'
import App from './App.vue'

// https://github.com/vuejs/core/issues/1696
// Convert the attributes of the given html element to an object.
// The values are transformed by applying JSON.parse
// in order to have correct types (Booleans, Numbers, Arrays, Objects, ...)
const attributesToProps = (element: Element) => {
  const props: any = {};
  for (let i = element.attributes.length - 1; i >= 0; i -= 1) {
      const attr = element.attributes[i];
      if (attr.name.startsWith('data-')) {
          const propName = attr.name.substring(5);
          props[propName] = JSON.parse(attr.value);
          element.removeAttribute(attr.name);
      }
  }
  return props;
};

const node = document.getElementById('app');
if (node != null) {
  const props = attributesToProps(node!);
  createApp(App, props).mount(node!);
}
