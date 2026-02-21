<template>
  <div class="app-root">
    <div class="app-header">
      <div class="app-logo">
        <div class="logo-box"></div>
        <div class="logo-text">CREATOR v2</div>
      </div>
      <div class="main-nav">
        <div 
          class="nav-tab" 
          :class="view == 'image' ? 'active' : ''"
          @click="set_view.set(\"image\")"
        >
          Vector Editor
        </div>
        <div 
          class="nav-tab" 
          :class="view == 'video' ? 'active' : ''"
          @click="set_view.set(\"video\")"
        >
          Video Editor
        </div>
        <div
          class="nav-tab"
          :class="view == 'css-demo' ? 'active' : ''"
          @click="set_view.set(\"css-demo\")"
        >
          CSS Demo
        </div>
      </div>
      <div class="system-status">
        <div class="status-dot"></div>
        <div class="status-text">GPU Ready</div>
      </div>
    </div>
    <div class="view-content">
      <Image v-if="view == 'image'" :open_file="open_file.clone()" />
      <Video v-if="view == 'video'" />
      <CssDemo v-if="view == 'css-demo'" />
    </div>
  </div>
</template>

<script>
    mod Image;
    mod Video;
    mod CssDemo;
    
    use crate::design::VectorFile;
    
    let (view, set_view) = crate::signals::create_signal("image".to_string());
    let open_file = Rc::new(RefCell::new(VectorFile {
      path: "assets/project.gemini".to_string(),
      objects: vec![],
    }));
</script>

<style>
.app-root {
  width: 100vw;
  height: 100vh;
  position: relative;
  flex-direction: column;
  background-color: #0c0c0c;
}

.app-header {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  z-index: 10;
  height: 56px;
  flex-direction: row;
  background-color: #1e1e1e;
  align-items: center;
  padding: 0 20px;
}

.app-logo {
  flex-direction: row;
  align-items: center;
  margin-right: 40px;
}

.logo-box {
  width: 24px;
  height: 24px;
  background-color: #3498db;
  margin-right: 12px;
  box-shadow: 0 0 8px rgba(52, 152, 219, 0.5);
}

.logo-text {
  font-size: 14px;
  font-weight: 700;
  color: #fff;
  letter-spacing: 0.1em;
}

.main-nav {
  height: 100%;
  flex-direction: row;
}

.nav-tab {
  height: 100%;
  padding: 0 24px;
  justify-content: center;
  font-size: 13px;
  font-weight: 500;
  color: #888;
  position: relative;
}

.nav-tab:hover {
  color: #ccc;
}

.nav-tab.active {
  color: #fff;
}

.nav-tab.active::after {
  //content: "";
  position: absolute;
  //bottom: 0;
  left: 0;
  height: 2px;
  background-color: #3498db;
}

.system-status {
  flex-direction: row;
  align-items: center;
  background-color: #252525;
  padding: 6px 12px;
  margin-left: 20px;
  border-width: 1px;
  border-color: #3a3a3a #333 #282828 #454545;
  outline-color: green;
  outline-width: 1px;
  outline-offset: 1px;
}

.status-dot {
  width: 8px;
  height: 8px;
  background-color: #2ecc71;
  margin-right: 8px;
  box-shadow: 0 0 10px rgba(46, 204, 113, 0.5);
}

.status-text {
  font-size: 10px;
  font-weight: 700;
  color: #2ecc71;
}

.view-content {
  position: absolute;
  top: 56px;
  left: 0;
  right: 0;
  bottom: 0;
  flex-direction: column;
}

.font-size-test {
  font-size: 100px;
  color: #f1c40f;
  margin: 20px;
}

.spacer {
  }
</style>
