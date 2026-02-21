<template>
  <div class="smart-shape-menu">
    <div
      class="smart-shape-trigger-hitbox"
      @mouseenter="{ set_shape_menu_open.set(true); }"
      @mouseleave="{ set_shape_menu_open.set(false); }"
    >
      <slot>
        <div class="smart-shape-trigger-icon">
          <img src="asset://phosphor/square.svg" class="smart-shape-icon-img" />
        </div>
      </slot>
    </div>

    <div
      class="smart-shape-flyout"
      v-show="shape_menu_open == true"
      @mouseenter="{ set_shape_menu_open.set(true); }"
      @mouseleave="{ set_shape_menu_open.set(false); }"
      @click="{ set_shape_menu_open.set(false); }"
    >
      <slot name="flyout">
        <div class="smart-shape-flyout-tool-icon">
          <img src="asset://phosphor/circle.svg" class="smart-shape-icon-img" />
        </div>
        <div class="smart-shape-flyout-tool-icon">
          <img src="asset://phosphor/line.svg" class="smart-shape-icon-img" />
        </div>
      </slot>
    </div>
  </div>
</template>

<script>
  let (shape_menu_open, set_shape_menu_open) = create_signal(false);
</script>

<style>
.smart-shape-menu {
  position: relative;
  width: 44px;
  height: 44px;
  margin-bottom: 6px;
}

.smart-shape-trigger-hitbox {
  width: 100%;
  height: 100%;
}

.smart-shape-trigger-icon {
  width: 100%;
  height: 100%;
  justify-content: center;
  align-items: center;
  border-width: 1px;
  border-color: transparent;
}

.smart-shape-trigger-icon:hover {
  background-color: #333;
}

.smart-shape-trigger-icon.active {
  background-color: #444;
  border-color: #555;
}

.smart-shape-flyout {
  position: absolute;
  top: 0;
  left: 100%;
  flex-direction: column;
  background-color: #1f1f1f;
  border-width: 1px;
  border-color: #3d3d3d;
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.45);
  z-index: 9500;
  padding: 4px;
}

.smart-shape-flyout-tool-icon {
  width: 40px;
  height: 40px;
  justify-content: center;
  align-items: center;
  border-width: 1px;
  border-color: transparent;
  margin-bottom: 4px;
}

.smart-shape-flyout-tool-icon:hover {
  background-color: #333;
}

.smart-shape-flyout-tool-icon.active {
  background-color: #444;
  border-color: #555;
}

.smart-shape-icon-img {
  width: 26px;
  height: 26px;
}
</style>
