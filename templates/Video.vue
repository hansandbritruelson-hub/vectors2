<template>
  <div class="video-editor">
    <div class="header">Video Editor</div>
    <div class="player-placeholder">
      <div class="label">Video Player Placeholder</div>
    </div>
    <div class="timeline">
      <div class="label">Timeline</div>
    </div>
  </div>
</template>

<style>
.video-editor {
  flex-direction: column;
  background-color: #1a1a1a;
  padding: 20px;
}
.header {
  color: #ffffff;
  font-size: 24px;
  margin-bottom: 20px;
}
.player-placeholder {
  height: 300px;
  background-color: #2a2a2a;
  justify-content: center;
  align-items: center;
  border-radius: 8px;
  margin-bottom: 20px;
}
.timeline {
  height: 100px;
  background-color: #333;
  justify-content: center;
  align-items: center;
  border-radius: 4px;
}
.label {
  color: #666;
}
</style>
