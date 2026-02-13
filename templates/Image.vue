<template>
  <div class="image-editor">
    <div class="header">Image Editor</div>
    <div class="input-row">
      <div class="label">Text:</div>
      <input 
        type="text" 
        :value="text_val" 
        @update:modelValue="|val| set_text_val.set(val)"
        class="my-input"
      />
    </div>
    <div class="input-row">
      <div class="label">Float:</div>
      <input 
        type="float64" 
        :value="float_val" 
        @update:modelValue="|val| set_float_val.set(val)"
        class="my-input"
      />
    </div>
    <div class="result">Text Value: {{ text_val }}</div>
    <div class="result">Float Value: {{ float_val }}</div>
  </div>
</template>

<script>
    let (text_val, set_text_val) = crate::signals::create_signal("Hello".to_string());
    let (float_val, set_float_val) = crate::signals::create_signal("0.0".to_string());
</script>

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
.input-row {
  flex-direction: row;
  align-items: center;
  margin-bottom: 10px;
}
.my-input {
  background-color: #232020ff;
  color: #ffffff;
  padding: 5px 10px;
  width: 200px;
  border-radius: 4px;
}
.label {
  color: #bbb;
  width: 80px;
}
.result {
  color: #888;
  margin-top: 10px;
}
</style>
