<template>
  <div class="container">
    <div class="nav">
      <div class="nav-item" @click="set_view.set('image')">Image</div>
      <div class="nav-item" @click="set_view.set('video')">Video</div>
    </div>
    
    <div class="content">
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
.container {
  flex-direction: column;
  background-color: #121212;
  width: 100%;
  height: 100%;
}
.nav {
  flex-direction: row;
  height: 60px;
  background-color: #1e1e1e;
  border-bottom: 1px solid #333;
}
.nav-item {
  padding: 0 20px;
  justify-content: center;
  align-items: center;
  color: #aaa;
  cursor: pointer;
}
.nav-item:hover {
  background-color: #2a2a2a;
  color: #fff;
}
.content {
  flex: 1;
}
</style>
