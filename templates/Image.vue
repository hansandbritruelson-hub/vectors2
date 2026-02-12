<template>
  <div class="image-editor">
    <div class="header">Image Editor</div>
    <div class="canvas-placeholder">
      <div class="label">Vector Content Placeholder</div>
    </div>
  </div>
</template>

<style>
.image-editor {
  flex-direction: column;
  background-color: #1a1a1a;
  padding: 20px;
}
.header {
  color: #ffffff;
  font-size: 24px;
  margin-bottom: 20px;
}
.canvas-placeholder {
  height: 400px;
  background-color: #2a2a2a;
  justify-content: center;
  align-items: center;
  border-radius: 8px;
}
.label {
  color: #666;
}
</style>
