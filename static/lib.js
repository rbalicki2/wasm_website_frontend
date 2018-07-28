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
  // type is a reserved word
  event_type: event.type,
  x: event.x,
  y: event.y,
});

const getPathFromChildToParent = (finalParent, node) => {
  const path = [];
  while (node && node !== finalParent) {
    // N.B. this excludes strings and such, but those can't have event handlers
    // so we're okay.
    path.push(Array.from(node.parentElement.childNodes).findIndex(x => x === node));
    node = node.parentElement;
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

    diff.forEach(([path, operation]) => {
      // this is how enum's are serialized...
      if (operation.Replace) {
        const node = findNodeWithPath(path);
        // LOL we should not be doing this
        node.nodeValue = operation.Replace.new_inner_html;
        node.innerHTML = operation.Replace.new_inner_html;
      } else if (operation.Insert) {
        const htmlToInsert = operation.Insert.new_inner_html;
        const node = findNodeWithPath(path.slice(0, path.length - 1));
        const lastPath = path[path.length - 1];
        if (lastPath === 0) {
          node.insertAdjacentHTML('afterbegin', htmlToInsert);
        } else {
          const childNode = node.childNodes[lastPath - 1];
          childNode.insertAdjacentHTML('afterend', htmlToInsert);
        }
      } else if (operation.Delete) {
        const node = findNodeWithPath(path);
        node.remove();
      }
    });
  });
}
  
export function initialize(id, appStateInterface) {
  appId = id;
  const appNode = getAppNode();
  appNode.innerHTML = '<div></div>';

  // OnClick
  appNode.addEventListener('click', (e) => {
    appStateInterface.handle_click(
      JSON.stringify(mouseEventToJson(e)),
      JSON.stringify(getPathFromChildToParent(appNode, e.target))
    );
    scheduleRender(appStateInterface);
  });

  // MouseOver
  appNode.addEventListener('mouseover', (e) => {
    // console.log(e);
  });

  scheduleRender(appStateInterface);
}
