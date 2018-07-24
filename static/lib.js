let appId;

const getAppNode = () => document.getElementById(appId);

const getPathFromChildToParent = (finalParent, node) => {
  const path = [];
  while (node && node !== finalParent) {
    // N.B. this excludes strings and such, but those can't have event handlers
    // so we're okay.
    path.push(Array.from(node.parentElement.children).findIndex(x => x === node));
    node = node.parentElement;
  }
  // N.B. if this is called with a node that is not a child of finalParent,
  // it will return a bogus path.
  // N.B. remove the leading [0] which is connects div id="app" to the top level component.
  // We don't care about that path.
  return path.reverse().slice(1);
};

function render(str) {
  const appNode = getAppNode();
  appNode.innerHTML = str;
}

function scheduleRender(appStateInterface) {
  setTimeout(() => render(appStateInterface.get_inner_html()));
}
  
export function initialize(id, appStateInterface) {
  appId = id;
  const appNode = getAppNode();

  // OnClick
  appNode.addEventListener('click', (e) => {
    console.log('click');
    appStateInterface.handle_click(
      JSON.stringify({
        shift_key: e.shiftKey,
      }),
      JSON.stringify(getPathFromChildToParent(appNode, e.target))
    );
    scheduleRender(appStateInterface);
  });

  scheduleRender(appStateInterface);
}
