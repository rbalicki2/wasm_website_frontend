let appId;

const getAppNode = () => document.getElementById(appId);

const mouseEventToJson = event => ({
  alt_key: event.altKey,
  client_x: event.clientX,
  client_y: event.clientY,
  ctrl_key: event.ctrlKey,
  layer_x: event.layerX,
  layer_y: event.layerY,
  meta_key: event.metaKey,
  movement_x: event.movementX,
  movement_y: event.movementY,
  offset_x: event.offsetX,
  offset_y: event.offsetY,
  page_x: event.pageX,
  page_y: event.pageY,
  screen_x: event.screenX,
  screen_y: event.screenY,
  shift_key: event.shiftKey,
  time_stamp: event.timeStamp,
  // type is a reserved word in Rust
  event_type: event.type,
  x: event.x,
  y: event.y,
});

const inputEventToJson = event => ({
  data: event.data,
  time_stamp: event.timeStamp,
  event_type: event.type,
  which: event.which,
  value: event.target.value,
});

const keyboardEventToJson = event => ({
  alt_key: event.altKey,
  char_code: event.charCode,
  code: event.code,
  ctrl_key: event.ctrlKey,
  key: event.key,
  key_code: event.keyCode,
  meta_key: event.metaKey,
  shift_key: event.shiftKey,
  time_stamp: event.timeStamp,
  event_type: event.type,
});

const htmlToElement = (html) => {
  var template = document.createElement('template');
  html = html.trim(); // Never return a text node of whitespace as the result
  template.innerHTML = html;
  return template.content.firstChild;
}
window.htmlToElement = htmlToElement;

const getPathFromChildToParent = (finalParent, node) => {
  const path = [];
  while (node && node.parentElement && node !== finalParent) {
    path.push(Array.from(node.parentElement.childNodes).findIndex(x => x === node));
    node = node.parentElement;
  }

  if (node !== finalParent) {
    // TODO don't do this
    return false;
  }
  // N.B. if this is called with a node that is not a child of finalParent,
  // it will return a bogus path.
  // N.B. remove the leading [0] which is connects div id="app" to the top level component.
  // We don't care about that path.
  return path.reverse().slice(1);
};

const findNodeWithPath = (path) =>
  path.reduce((node, childIndex) => (node && node.childNodes[childIndex]), getAppNode());

function scheduleRender(appStateInterface) {
  setTimeout(() => {
    const diff = JSON.parse(appStateInterface.get_diff());
    const nodesToRemove = [];
    diff.forEach(([path, operation]) => {
      // this is how enum's are serialized...
      if (operation.Replace) {
        const htmlToInsert = operation.Replace.new_inner_html;
        const parentNode = findNodeWithPath(path.slice(0, path.length - 1));
        const lastPath = path[path.length - 1];
        const childNode = parentNode.childNodes[lastPath];

        parentNode.insertBefore(htmlToElement(htmlToInsert), childNode);
        childNode.remove();
      } else if (operation.Insert) {
        const htmlToInsert = operation.Insert.new_inner_html;
        const parentNode = findNodeWithPath(path.slice(0, path.length - 1));
        const lastPath = path[path.length - 1];
        const isLastElement = lastPath === parentNode.childNodes.length;
        if (lastPath === 0) {
          parentNode.insertAdjacentHTML('afterbegin', htmlToInsert);
        } else if (isLastElement) {
          parentNode.insertAdjacentHTML('beforeend', htmlToInsert);
        } else {
          const childNode = parentNode.childNodes[lastPath - 1];
          parentNode.insertBefore(htmlToElement(htmlToInsert), childNode);
        }
      } else if (operation.Delete) {
        const node = findNodeWithPath(path);
        // node.remove();
        nodesToRemove.push(node);
      } else if (operation.UpdateAttributes) {
        const node = findNodeWithPath(path);
        Object.entries(operation.UpdateAttributes.new_attributes)
          .forEach(([attr, val]) => {
            node[attr] = val;
          });
      }
    });

    // This seems like a smell, like maybe we should just be getting the right
    // diff.
    nodesToRemove.forEach(node => node.remove());
  });
}

export function initialize(id, appStateInterface) {
  appId = id;
  const appNode = getAppNode();
  
  appNode.addEventListener('click', (e) => {
    const shouldUpdate = appStateInterface.handle_click(
      JSON.stringify(mouseEventToJson(e)),
      JSON.stringify(getPathFromChildToParent(appNode, e.target))
    );
    if (shouldUpdate) { scheduleRender(appStateInterface); }
  });

  // MouseOver
  appNode.addEventListener('mouseover', (e) => {
    const shouldUpdate = appStateInterface.handle_mouseover(
      JSON.stringify(mouseEventToJson(e)),
      JSON.stringify(getPathFromChildToParent(appNode, e.target))
    );
    if (shouldUpdate) { scheduleRender(appStateInterface); }
  });

  // MouseOut
  appNode.addEventListener('mouseout', (e) => {
    const shouldUpdate = appStateInterface.handle_mouseout(
      JSON.stringify(mouseEventToJson(e)),
      JSON.stringify(getPathFromChildToParent(appNode, e.target))
    );
    if (shouldUpdate) { scheduleRender(appStateInterface); }
  });

  // Input
  appNode.addEventListener('input', (e) => {
    const shouldUpdate = appStateInterface.handle_input(
      JSON.stringify(inputEventToJson(e)),
      JSON.stringify(getPathFromChildToParent(appNode, e.target))
    );
    if (shouldUpdate) { scheduleRender(appStateInterface); }
  });

  appNode.addEventListener('keydown', (e) => {
    const shouldUpdate = appStateInterface.handle_keydown(
      JSON.stringify(keyboardEventToJson(e)),
      JSON.stringify(getPathFromChildToParent(appNode, e.target))
    );
    if (shouldUpdate) { scheduleRender(appStateInterface); }
  });

  scheduleRender(appStateInterface);
}
