export function initMenu(interfaces, callback) {

  console.log('interfaces', interfaces);
  Object.keys(interfaces).forEach(moduleName => {
    const modulei = interfaces[moduleName];
    const menubar = document.getElementById('menubar');
    const menuItem = document.createElement('div');

    const menuLabel = document.createElement('span');
    menuLabel.setAttribute('class', 'menuLabel');
    menuLabel.innerText = moduleName;

    const menuDropDown = document.createElement('div');

    modulei.forEach(functioni => {
      const link = document.createElement('a');
      link.href = '#';
      link.innerText = functioni.signature;
      link.onclick = () => {
        callback(moduleName, functioni);
      }

      menuDropDown.appendChild(link);
    })

    menuItem.appendChild(menuLabel);
    menuItem.appendChild(menuDropDown);
    menubar.appendChild(menuItem);
  })

  // https://github.com/morgan3d/misc/tree/master/jsmenu

  // Polyfill
  if (HTMLCollection.prototype.forEach === undefined) { HTMLCollection.prototype.forEach = [].forEach }

  function showMenu(e) { closeAllMenus(); this.classList.add('activeMenu'); e.stopPropagation() }

  function closeAllMenus() { document.getElementsByClassName('activeMenu').forEach(function (node) { node.classList.remove('activeMenu') }) }

  (function() {function on(e,f){window.addEventListener(e,f)}

  // Show menu event handler
  on('load', function (event) { document.getElementsByClassName('menuLabel').forEach(function(node) { node.onclick = showMenu }) })

  // Close all open menus on click or escape
  on('click', closeAllMenus, true)
  on('keydown', function (event) { (event.key === "Escape") && closeAllMenus() }, true)

  var style = document.createElement('style')
  style.type = 'text/css'
  style.innerHTML =`
  #m {position:absolute;top:43px;font-size:13px;user-select:none}

  #m div{display:inline-block;vertical-align:top}

  /* Menu label (no hover) */
  #m .menuLabel{padding:4px 8px 4px 8px;margin-right:10px;margin-left:-8px}

  /* Menu label (hover) */
  #m .menuLabel:hover{background:#eee}

  /* Drop-down menu */
  #m div div{position:absolute;display:block;min-width:120px;overflow:auto;background:#fff;box-shadow:0px 2px 6px 0px rgba(0,0,0,.5);z-index:4;padding:4px 2px 2px 30px;margin-left:-8px;margin-top:4px}

  /* Drop-down menu (hidden) */
  #m span:not(.activeMenu) + div{visibility:hidden}

  /* Menu item */
  #m div div a{color:#000;text-decoration:none;display:block;padding:4px;margin:4px 0 4px}

  /* Left-aligned icon within a menu item */
  #m div div a .icon{position:absolute;left:9px}

  /* Hotkey documentation in a menu item */
  #m div div a .hotkey{position:absolute;right:9px;color:rgba(0,0,0,.5)}

  /* Google's icon font within the menu */
  #m div div a .material-icons{font-size:120%;color:rgba(0,0,0,.55)}

  /* Line between menu items */
  #m div div hr{border:none;height:1px;background:rgba(0,0,0,.15);margin:5px -2px 5px -30px}`.replace(/#m /g, '#menubar ')
  document.getElementsByTagName('head')[0].appendChild(style)
  })();
}
