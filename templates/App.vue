<template>
  <div class="container">
    <div class="nav">
      <div class="nav-item" @click="set_view.set('image')">Image</div>
      <div class="nav-item" @click="set_view.set('video')">Video</div>
    </div>
    
    <div class="content">
      <div class="test-box">
        <div class="inner-box"></div>
      </div>
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
  background-color: #1515eeff;
  padding: 10px;
}
.nav-item {
  background: #2aae23ff;
  margin-right: 15px;
  padding: 20px;
  border: 2px solid #1515eeff;
  outline: 2px solid #ae9b23ff;
  box-shadow: 4px 4px 3px #c41a1a80;
}
.nav-item:hover {
  background: #27bb67ff;
}
.content {
  flex-direction: column;
  background-color: #0f0f12;
  padding: 30px;
}
.test-box {
  width: 200px;
  background: #333;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0px 0px 40px 10px #ff0000; /* Red glow shadow */
}
.inner-box {
  width: 100%;
  height: 50px;
  background: #555;
  box-shadow: inset 5px 5px 10px #000000; /* Inset shadow TEST (won't work yet, but let's see) */
}
</style>
