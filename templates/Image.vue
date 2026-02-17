<template>
  <div class="editor-container">
    <div class="sidebar">
      <div class="tool-icon active">
        <img src="asset://phosphor/selection.svg" class="icon-img" />
      </div>
      <div class="tool-icon">
        <img src="asset://phosphor/cursor.svg" class="icon-img" />
      </div>
      <div class="tool-icon">
        <img src="asset://phosphor/pencil.svg" class="icon-img" />
      </div>
      <div class="tool-icon">
        <img src="asset://phosphor/square.svg" class="icon-img" />
      </div>
      <div class="tool-icon">
        <img src="asset://phosphor/circle.svg" class="icon-img" />
      </div>
      <div class="tool-icon">
        <img src="asset://phosphor/hand-grabbing.svg" class="icon-img" />
      </div>
      <div class="spacer"></div>
      <div class="tool-icon">
        <img src="asset://phosphor/settings.svg" class="icon-img" />
      </div>
    </div>

    <div class="main-content">
      <div class="top-bar">
        <div
          class="file-menu-container"
          @mouseenter="{ renderer_core::log(\"menu: file mouseenter\"); set_file_menu_open.set(true); }"
          @mouseleave="{ renderer_core::log(\"menu: file mouseleave\"); set_file_menu_open.set(false); }"
        >
          <div class="menu-item file-menu-trigger">
            File
          </div>
          <div class="file-menu-dropdown" v-if="file_menu_open == true">
            <div class="file-menu-command" @click="{ renderer_core::log(\"menu: command new file\"); }">New File</div>
            <div class="file-menu-command" @click="{ renderer_core::log(\"menu: command open\"); }">Open...</div>
            <div class="file-menu-command" @click="{ renderer_core::log(\"menu: command save\"); }">Save</div>
            <div class="file-menu-command" @click="{ renderer_core::log(\"menu: command export\"); }">Export</div>
          </div>
        </div>
        <div class="menu-item">Edit</div>
        <div class="menu-item">View</div>
        <div class="menu-item">Object</div>
        <div class="spacer"></div>
        <div class="project-title">Untitled Vector Project</div>
        <div class="spacer"></div>
        <div class="user-profile">HB</div>
      </div>
      <div class="context-bar">
        <div class="context-tools">
          <div class="tool-icon-small">
            <img src="asset://phosphor/undo.svg" class="icon-img-small" />
          </div>
          <div class="tool-icon-small">
            <img src="asset://phosphor/redo.svg" class="icon-img-small" />
          </div>
          <div class="divider"></div>
          <div class="tool-icon-small">
            <img src="asset://phosphor/copy.svg" class="icon-img-small" />
          </div>
          <div class="tool-icon-small">
            <img src="asset://phosphor/paste.svg" class="icon-img-small" />
          </div>
          <div class="tool-icon-small">
            <img src="asset://phosphor/delete.svg" class="icon-img-small" />
          </div>
        </div>
        <div class="spacer"></div>
        <div class="snap-tools">
          <div class="tool-icon-small active">
            <img src="asset://phosphor/snapping.svg" class="icon-img-small" />
          </div>
          <div class="label-small">Snapping</div>
        </div>
      </div>

      <div class="editor-body">
        <div class="canvas-area">
          <div class="canvas-mock">
            <div class="rect-shape"></div>
            <div class="circle-shape"></div>
          </div>
        </div>

        <div class="right-panel">
          <div class="panel-header">
            <img src="asset://phosphor/layers.svg" class="icon-img-mini" />
            <div class="panel-title">Layers</div>
          </div>
          <div class="layer-list">
            <div class="layer-item active">
              <img src="asset://phosphor/circle.svg" class="icon-img-mini" />
              <div class="layer-name">[[ props.design.path ]]</div>
            </div>
            <div class="layer-item">
              <img src="asset://phosphor/square.svg" class="icon-img-mini" />
              <div class="layer-name">Rectangle 1</div>
            </div>
            <div class="layer-item">
              <img src="asset://phosphor/pencil.svg" class="icon-img-mini" />
              <div class="layer-name">Path 1</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
  use crate::design::Design;
  let (file_menu_open, set_file_menu_open) = crate::signals::create_signal(false);

  pub struct Props {
    pub design: Rc<Design>
  }
</script>

<style>
.editor-container {
  width: 100%;
  height: 100%;
  flex-direction: row;
  background-color: #121212;
  color: #e0e0e0;
}

/* Sidebar */
.sidebar {
  width: 64px;
  flex-direction: column;
  background-color: #1e1e1e;
  border-right: 1px solid #333;
  padding: 12px 0;
  align-items: center;
}

.tool-icon {
  width: 44px;
  height: 44px;
  justify-content: center;
  align-items: center;
  border-width: 1px;
  border-color: transparent;
  margin-bottom: 6px;
}

.tool-icon:hover {
  background-color: #333;
}

.tool-icon.active {
  background-color: #444;
  border-color: #555;
}

.icon-img {
  width: 28px;
  height: 28px;
}

/* Main Content */
.main-content {
    flex-direction: column;
}

/* Top Bar */
.top-bar {
  height: 44px;
  flex-direction: row;
  background-color: #1e1e1e;
  align-items: center;
  padding: 0 16px;
}

.file-menu-container {
  position: relative;
  height: 100%;
  justify-content: center;
}

.menu-item {
  padding: 0 12px;
  height: 100%;
  justify-content: center;
  font-size: 13px;
  color: #bbb;
}

.file-menu-trigger {
  z-index: 5000;
}

.menu-item:hover {
  color: #fff;
  background-color: #333;
}

.file-menu-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  min-width: 170px;
  flex-direction: column;
  background-color: #242424;
  border-width: 1px;
  border-color: #3d3d3d;
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.45);
  z-index: 9000;
  padding: 4px 0;
}

.file-menu-command {
  height: 30px;
  justify-content: center;
  padding: 0 12px;
  color: #d0d0d0;
  font-size: 12px;
}

.file-menu-command:hover {
  background-color: #3a3a3a;
  color: #ffffff;
}

.project-title {
  font-size: 13px;
  //font-weight: 500;
  color: #888;
}

/* Context Bar */
.context-bar {
  height: 48px;
  flex-direction: row;
  background-color: #252525;
  align-items: center;
  padding: 0 12px;
}

.context-tools {
  flex-direction: row;
  align-items: center;
}

.tool-icon-small {
  width: 32px;
  height: 32px;
  justify-content: center;
  align-items: center;
  margin: 0 2px;
}

.tool-icon-small:hover {
  background-color: #383838;
}

.tool-icon-small.active {
  background-color: #444;
}

.icon-img-small {
  width: 18px;
  height: 18px;
}

.divider {
  width: 1px;
  height: 20px;
  background-color: #444;
  margin: 0 8px;
}

.snap-tools {
  flex-direction: row;
  align-items: center;
  padding: 4px 8px;
  background-color: #333;
  border-width: 1px;
  border-color: #444;
}

.label-small {
  font-size: 11px;
  margin-left: 6px;
  color: #ccc;
}

/* Editor Body */
.editor-body {
    flex-direction: row;
}

.canvas-area {
  background-color: #121212;
  justify-content: center;
  align-items: center;
  padding: 40px;
}

.canvas-mock {
  width: 600px;
  height: 400px;
  background-color: #fff;
  box-shadow: 0 10px 30px rgba(0,0,0,0.5);
  position: relative;
  /**overflow: hidden;**/}

.rect-shape {
  position: absolute;
  top: 50px;
  left: 50px;
  width: 150px;
  height: 100px;
  background-color: #3498db;
  border: 2px solid #2980b9;
}

.circle-shape {
  position: absolute;
  top: 180px;
  left: 300px;
  width: 120px;
  height: 120px;
  background-color: #e74c3c;
  border: 2px solid #c0392b;
}

/* Right Panel */
.right-panel {
  width: 240px;
  background-color: #1e1e1e;
  border-left: 1px solid #333;
  flex-direction: column;
}

.panel-header {
  height: 40px;
  flex-direction: row;
  align-items: center;
  padding: 0 12px;
  background-color: #252525;
  }

.panel-title {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  color: orange;
  letter-spacing: 0.05em;
  margin-left: 8px;
}

.layer-list {
  flex-direction: column;
  padding: 8px;
}

.layer-item {
  height: 32px;
  flex-direction: row;
  align-items: center;
  padding: 0 8px;
  margin-bottom: 2px;
}

.layer-item:hover {
  background-color: #2a2a2a;
}

.layer-item.active {
  background-color: #333;
  border: 1px solid #444;
}

.layer-name {
  font-size: 12px;
  margin-left: 8px;
  color: #ccc;
}

.icon-img-mini {
  width: 14px;
  height: 14px;
}

.spacer {
  }
</style>
