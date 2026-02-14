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
          @click="set_view.set('image')"
        >
          Vector Editor
        </div>
        <div 
          class="nav-tab" 
          :class="view == 'video' ? 'active' : ''"
          @click="set_view.set('video')"
        >
          Video Editor
        </div>
      </div>
      <div class="spacer"></div>
      <div class="system-status">
        <div class="status-dot"></div>
        <div class="status-text">GPU ACTIVE</div>
      </div>
    </div>
    
    <div class="view-content">
      <Image v-if="view == 'image'" />
      <Video v-if="view == 'video'" />
    </div>
  </div>
</template>

<script>
    mod Image;
    mod Video;
    
    let (view, set_view) = crate::signals::create_signal("image".to_string());
</script>

<style>
.app-root {
  width: 100%;
  height: 100%;
  flex-direction: column;
  background-color: #0c0c0c;
  font-family: 'Inter', system-ui, sans-serif;
}

.app-header {
  height: 56px;
  flex-direction: row;
  background-color: #1e1e1e;
  border-bottom: 1px solid #333;
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
  border-radius: 4px;
  margin-right: 12px;
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
  content: "";
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2px;
  background-color: #3498db;
}

.system-status {
  flex-direction: row;
  align-items: center;
  background-color: #252525;
  padding: 6px 12px;
  border-radius: 20px;
  border: 1px solid #333;
}

.status-dot {
  width: 8px;
  height: 8px;
  background-color: #2ecc71;
  border-radius: 4px;
  margin-right: 8px;
  box-shadow: 0 0 10px rgba(46, 204, 113, 0.5);
}

.status-text {
  font-size: 10px;
  font-weight: 700;
  color: #2ecc71;
}

.view-content {
  flex-grow: 1;
  flex-direction: column;
}

.spacer {
  flex-grow: 1;
}
</style>
