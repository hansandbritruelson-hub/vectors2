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
  background: #0a0a0c;
  width: 100%;
  height: 100%;
}
.nav {
  flex-direction: row;
  height: 60px;
  background-color: #1515eeff;
}
.nav-item {
  width: 100px;
  height: 60px;
  background: #2aae23ff;
}
.nav-item:hover {
  background: #27bb67ff;
}
.content {
  flex-direction: column;
  background-color: #0f0f12;
}
</style>
