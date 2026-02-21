<template>
  <div class="editor-container">
    <div class="sidebar">
      <div
        v-if="active_tool == TOOL_SELECT"
        class="tool-icon active"
        @click="{ set_active_tool.set(TOOL_SELECT.to_string()); set_active_path_id.set(String::new()); }"
      >
        <img src="asset://phosphor/selection.svg" class="icon-img" />
      </div>
      <div
        v-if="active_tool != TOOL_SELECT"
        class="tool-icon"
        @click="{ set_active_tool.set(TOOL_SELECT.to_string()); set_active_path_id.set(String::new()); }"
      >
        <img src="asset://phosphor/selection.svg" class="icon-img" />
      </div>

      <div class="tool-icon">
        <img src="asset://phosphor/cursor.svg" class="icon-img" />
      </div>

      <div
        v-if="active_tool == TOOL_BEZIER"
        class="tool-icon active"
        @click="{ set_active_tool.set(TOOL_BEZIER.to_string()); }"
      >
        <img src="asset://phosphor/pencil.svg" class="icon-img" />
      </div>
      <div
        v-if="active_tool != TOOL_BEZIER"
        class="tool-icon"
        @click="{ set_active_tool.set(TOOL_BEZIER.to_string()); }"
      >
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
        <div class="file-menu-container">
          <div
            class="menu-item file-menu-trigger"
            @mouseenter="{ set_file_menu_open.set(true); }"
            @mouseleave="{ set_file_menu_open.set(false); }"
          >
            File
          </div>
          <div
            class="file-menu-dropdown"
            v-show="file_menu_open == true"
            @mouseenter="{ set_file_menu_open.set(true); }"
            @mouseleave="{ set_file_menu_open.set(false); }"
          >
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
        <div class="project-title">{{ props.open_file.borrow().path.clone() }}</div>
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
          <div
            v-if="snapping_enabled == true"
            class="tool-icon-small active"
            @click="{ set_snapping_enabled.set(false); }"
          >
            <img src="asset://phosphor/snapping.svg" class="icon-img-small" />
          </div>
          <div
            v-if="snapping_enabled != true"
            class="tool-icon-small"
            @click="{ set_snapping_enabled.set(true); }"
          >
            <img src="asset://phosphor/snapping.svg" class="icon-img-small" />
          </div>
          <div class="label-small">Snapping</div>
        </div>
      </div>

      <div class="editor-body">
        <div class="canvas-area">
          <div
            class="canvas-mock"
            @click="{ (on_canvas_click)(event); }"
          >
            <div class="path-layer">
              <div v-for="obj in objects" class="path-shape" :d="obj.path_data.clone()"></div>
              <div v-for="marker in point_markers" class="point-marker" :d="marker.path_data.clone()"></div>
            </div>
          </div>
        </div>

        <div class="right-panel">
          <div class="panel-header">
            <img src="asset://phosphor/layers.svg" class="icon-img-mini" />
            <div class="panel-title">Objects</div>
          </div>
          <div class="layer-list">
            <div class="layer-item" v-for="obj in objects">
              <img src="asset://phosphor/pencil.svg" class="icon-img-mini" />
              <div class="layer-name">{{ format!("{} (z={:.0})", obj.name, obj.z_index) }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
  use crate::design::{VectorFile, VectorObject, VectorPoint};

  const TOOL_SELECT: &str = "select";
  const TOOL_BEZIER: &str = "bezier";
  const SNAP_DISTANCE: f32 = 12.0;
  const BASE_OBJECT_Z_INDEX: f32 = 10.0;

  #[derive(Clone)]
  pub struct PointMarker {
    pub id: String,
    pub path_data: String,
  }

  pub struct Props {
    pub open_file: Rc<RefCell<VectorFile>>,
  }

  fn build_path_data(points: &[VectorPoint], closed: bool) -> String {
    if points.is_empty() {
      return String::new();
    }

    let mut path_data = format!("M {:.2} {:.2}", points[0].x, points[0].y);
    for point in points.iter().skip(1) {
      path_data.push_str(&format!(" L {:.2} {:.2}", point.x, point.y));
    }

    if closed {
      path_data.push_str(" Z");
    }

    path_data
  }

  fn build_point_marker_path(point: &VectorPoint) -> String {
    let size = 3.0;
    format!(
      "M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
      point.x - size,
      point.y - size,
      point.x + size,
      point.y - size,
      point.x + size,
      point.y + size,
      point.x - size,
      point.y + size,
    )
  }

  fn build_point_markers(objects: &[VectorObject]) -> Vec<PointMarker> {
    let mut markers = vec![];

    for object in objects {
      if object.object_type != "path" {
        continue;
      }

      for point in &object.points {
        markers.push(PointMarker {
          id: format!("{}-{}", object.id, point.id),
          path_data: build_point_marker_path(point),
        });
      }
    }

    markers
  }

  fn next_path_number(objects: &[VectorObject]) -> usize {
    objects.iter().filter(|object| object.object_type == "path").count() + 1
  }

  fn next_z_index(objects: &[VectorObject]) -> f32 {
    let mut max_z = BASE_OBJECT_Z_INDEX - 1.0;
    for object in objects {
      if object.z_index > max_z {
        max_z = object.z_index;
      }
    }
    max_z + 1.0
  }

  let (file_menu_open, set_file_menu_open) = crate::signals::create_signal(false);
  let (active_tool, set_active_tool) = crate::signals::create_signal(TOOL_SELECT.to_string());
  let (snapping_enabled, set_snapping_enabled) = crate::signals::create_signal(true);
  let (active_path_id, set_active_path_id) = crate::signals::create_signal(String::new());

  let initial_objects = props.open_file.borrow().objects.clone();
  let (objects, set_objects) = crate::signals::create_signal::<Vec<VectorObject>>(initial_objects.clone());
  let (point_markers, set_point_markers) = crate::signals::create_signal::<Vec<PointMarker>>(build_point_markers(&initial_objects));

  let open_file_ref = props.open_file.clone();
  let commit_objects = Rc::new(move |next_objects: Vec<VectorObject>| {
    let next_markers = build_point_markers(&next_objects);
    set_point_markers.set(next_markers);
    set_objects.set(next_objects.clone());
    open_file_ref.borrow_mut().objects = next_objects;
  });

  let engine_ref = engine.clone();
  let on_canvas_click = Rc::new(move |event: renderer_core::UiEvent| {
    if active_tool.get() != TOOL_BEZIER {
      return;
    }

    let (canvas_x, canvas_y) = engine_ref.borrow().get_node_final_position(event.current_target.id);
    let mut click_x = event.mouse_x - canvas_x;
    let mut click_y = event.mouse_y - canvas_y;

    let mut next_objects = objects.get();
    let current_path_id = active_path_id.get();

    if current_path_id.is_empty() {
      let path_number = next_path_number(&next_objects);
      let new_id = format!("path-{}", path_number);
      let mut new_path = VectorObject {
        id: new_id.clone(),
        name: format!("Path {}", path_number),
        object_type: "path".to_string(),
        z_index: next_z_index(&next_objects),
        closed: false,
        points: vec![VectorPoint {
          id: "pt-1".to_string(),
          x: click_x,
          y: click_y,
        }],
        path_data: String::new(),
      };
      new_path.path_data = build_path_data(&new_path.points, false);
      next_objects.push(new_path);
      commit_objects(next_objects);
      set_active_path_id.set(new_id);
      return;
    }

    if let Some(path_object) = next_objects.iter_mut().find(|object| object.id == current_path_id) {
      let can_close = path_object.points.len() >= 2;
      let mut should_close = false;

      if snapping_enabled.get() && can_close {
        let first = &path_object.points[0];
        let dx = click_x - first.x;
        let dy = click_y - first.y;
        if (dx * dx) + (dy * dy) <= (SNAP_DISTANCE * SNAP_DISTANCE) {
          click_x = first.x;
          click_y = first.y;
          should_close = true;
        }
      }

      if should_close {
        path_object.closed = true;
        path_object.path_data = build_path_data(&path_object.points, true);
        commit_objects(next_objects);
        set_active_path_id.set(String::new());
        return;
      }

      let next_point_number = path_object.points.len() + 1;
      path_object.points.push(VectorPoint {
        id: format!("pt-{}", next_point_number),
        x: click_x,
        y: click_y,
      });
      path_object.path_data = build_path_data(&path_object.points, false);
      commit_objects(next_objects);
    }
  });
</script>

<style>
.editor-container {
  width: 100%;
  height: 100%;
  flex-direction: row;
  background-color: #121212;
  color: #e0e0e0;
}

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

.main-content {
  flex-direction: column;
}

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
  color: #888;
}

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
  background-color: #ffffff;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
  position: relative;
}

.path-layer {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
}

.path-shape {
  fill: transparent;
  stroke: #ff2d2d;
  stroke-width: 2px;
}

.point-marker {
  fill: #ff2d2d;
  stroke: transparent;
  stroke-width: 0px;
}

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
