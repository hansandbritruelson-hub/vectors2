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
        <img src="asset://phosphor/bezier-curve.svg" class="icon-img" />
      </div>
      <div
        v-if="active_tool != TOOL_BEZIER"
        class="tool-icon"
        @click="{ set_active_tool.set(TOOL_BEZIER.to_string()); }"
      >
        <img src="asset://phosphor/bezier-curve.svg" class="icon-img" />
      </div>

      <SmartShapeMenuItem>
        <div
          v-if="active_tool == TOOL_SHAPE_RECT"
          class="smart-shape-trigger-icon active"
          @click="{ set_active_path_id.set(String::new()); set_active_tool.set(TOOL_SHAPE_RECT.to_string()); renderer_core::log(\"tool: shape-rect\"); }"
        >
          <img src="asset://phosphor/square.svg" class="smart-shape-icon-img" />
        </div>
        <div
          v-if="active_tool != TOOL_SHAPE_RECT"
          class="smart-shape-trigger-icon"
          @click="{ set_active_path_id.set(String::new()); set_active_tool.set(TOOL_SHAPE_RECT.to_string()); renderer_core::log(\"tool: shape-rect\"); }"
        >
          <img src="asset://phosphor/square.svg" class="smart-shape-icon-img" />
        </div>

        <template #flyout>
          <div
            v-if="active_tool == TOOL_SHAPE_CIRCLE"
            class="smart-shape-flyout-tool-icon active"
            @click="{ set_active_path_id.set(String::new()); set_active_tool.set(TOOL_SHAPE_CIRCLE.to_string()); renderer_core::log(\"tool: shape-circle\"); }"
          >
            <img src="asset://phosphor/circle.svg" class="smart-shape-icon-img" />
          </div>
          <div
            v-if="active_tool != TOOL_SHAPE_CIRCLE"
            class="smart-shape-flyout-tool-icon"
            @click="{ set_active_path_id.set(String::new()); set_active_tool.set(TOOL_SHAPE_CIRCLE.to_string()); renderer_core::log(\"tool: shape-circle\"); }"
          >
            <img src="asset://phosphor/circle.svg" class="smart-shape-icon-img" />
          </div>

          <div
            v-if="active_tool == TOOL_SHAPE_LINE"
            class="smart-shape-flyout-tool-icon active"
            @click="{ set_active_path_id.set(String::new()); set_active_tool.set(TOOL_SHAPE_LINE.to_string()); renderer_core::log(\"tool: shape-line\"); }"
          >
            <img src="asset://phosphor/line.svg" class="smart-shape-icon-img" />
          </div>
          <div
            v-if="active_tool != TOOL_SHAPE_LINE"
            class="smart-shape-flyout-tool-icon"
            @click="{ set_active_path_id.set(String::new()); set_active_tool.set(TOOL_SHAPE_LINE.to_string()); renderer_core::log(\"tool: shape-line\"); }"
          >
            <img src="asset://phosphor/line.svg" class="smart-shape-icon-img" />
          </div>
        </template>
      </SmartShapeMenuItem>
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
            @mousedown="{ (on_canvas_mouse_down)(event); }"
            @mousemove="{ (on_canvas_mouse_move)(event); }"
            @mouseup="{ (on_canvas_mouse_up)(event); }"
          >
            <div class="path-layer">
              <div class="bezier-guide-overlay" :d="guide_overlay_path.clone()"></div>
              <div class="bezier-curve-overlay" :d="curve_overlay_path.clone()"></div>
              <div class="bezier-handle-marker-overlay" :d="handle_marker_path.clone()"></div>
              <div class="bezier-anchor-marker-overlay" :d="anchor_marker_path.clone()"></div>
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
  mod SmartShapeMenuItem;

  use crate::design::{VectorFile, VectorHandle, VectorObject, VectorPoint};

  const TOOL_SELECT: &str = "select";
  const TOOL_BEZIER: &str = "bezier";
  const TOOL_SHAPE_RECT: &str = "shape-rect";
  const TOOL_SHAPE_CIRCLE: &str = "shape-circle";
  const TOOL_SHAPE_LINE: &str = "shape-line";
  const SNAP_DISTANCE: f32 = 12.0;
  const BASE_OBJECT_Z_INDEX: f32 = 10.0;
  const HANDLE_RATIO: f32 = 0.3;
  const MAX_HANDLE_LENGTH: f32 = 64.0;
  const PEN_DRAG_THRESHOLD_SQ: f32 = 16.0;

  #[derive(Clone)]
  pub struct PointMarker {
    pub id: String,
    pub path_data: String,
  }

  #[derive(Clone, Default)]
  pub struct BezierOverlayPaths {
    pub curve_path_data: String,
    pub guide_path_data: String,
    pub anchor_marker_path_data: String,
    pub handle_marker_path_data: String,
  }

  #[derive(Clone, Debug)]
  struct PenDragPoint {
    path_id: String,
    point_index: usize,
  }

  enum PenInsertResult {
    NewPathStarted { path_id: String, point_index: usize },
    PointAdded { path_id: String, point_index: usize },
    PathClosed,
    NoOp,
  }

  pub struct Props {
    pub open_file: Rc<RefCell<VectorFile>>,
  }

  fn build_path_data(points: &[VectorPoint], closed: bool) -> String {
    if points.is_empty() {
      return String::new();
    }

    fn append_segment(path_data: &mut String, from: &VectorPoint, to: &VectorPoint) {
      let has_curve = from.handle_out.is_some() || to.handle_in.is_some();
      if has_curve {
        let (c1_x, c1_y) = if let Some(handle) = &from.handle_out {
          (handle.x, handle.y)
        } else {
          (from.x, from.y)
        };
        let (c2_x, c2_y) = if let Some(handle) = &to.handle_in {
          (handle.x, handle.y)
        } else {
          (to.x, to.y)
        };
        path_data.push_str(&format!(
          " C {:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
          c1_x, c1_y, c2_x, c2_y, to.x, to.y
        ));
      } else {
        path_data.push_str(&format!(" L {:.2} {:.2}", to.x, to.y));
      }
    }

    let mut path_data = format!("M {:.2} {:.2}", points[0].x, points[0].y);
    for index in 1..points.len() {
      append_segment(&mut path_data, &points[index - 1], &points[index]);
    }

    if closed {
      if points.len() > 1 {
        append_segment(&mut path_data, &points[points.len() - 1], &points[0]);
      }
      path_data.push_str(" Z");
    }

    path_data
  }

  fn build_square_marker_path(x: f32, y: f32, size: f32) -> String {
    format!(
      "M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
      x - size,
      y - size,
      x + size,
      y - size,
      x + size,
      y + size,
      x - size,
      y + size,
    )
  }

  fn append_square_marker_path(path_data: &mut String, x: f32, y: f32, size: f32) {
    path_data.push_str(&format!(
      " M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
      x - size,
      y - size,
      x + size,
      y - size,
      x + size,
      y + size,
      x - size,
      y + size,
    ));
  }

  fn append_line_stamps(path_data: &mut String, x0: f32, y0: f32, x1: f32, y1: f32, width: f32) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len_sq = (dx * dx) + (dy * dy);
    let half = (width * 0.6).max(1.0);
    if len_sq <= 0.0001 {
      append_square_marker_path(path_data, x0, y0, half);
      return;
    }
    let len = len_sq.sqrt();
    let spacing = (width * 0.45).clamp(1.0, 2.0);
    let steps = ((len / spacing).ceil() as usize).max(1);
    for i in 0..=steps {
      let t = (i as f32) / (steps as f32);
      append_square_marker_path(path_data, x0 + (dx * t), y0 + (dy * t), half);
    }
  }

  fn cubic_point_xy(
    t: f32,
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
  ) -> (f32, f32) {
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;
    let b0 = mt2 * mt;
    let b1 = 3.0 * mt2 * t;
    let b2 = 3.0 * mt * t2;
    let b3 = t2 * t;
    (
      (p0.0 * b0) + (p1.0 * b1) + (p2.0 * b2) + (p3.0 * b3),
      (p0.1 * b0) + (p1.1 * b1) + (p2.1 * b2) + (p3.1 * b3),
    )
  }

  fn estimate_cubic_steps(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> usize {
    let chord = distance(p0.0, p0.1, p3.0, p3.1);
    let control_net =
      distance(p0.0, p0.1, p1.0, p1.1) +
      distance(p1.0, p1.1, p2.0, p2.1) +
      distance(p2.0, p2.1, p3.0, p3.1);
    let curvature = (control_net - chord).max(0.0);
    let approx_len = control_net.max(chord);
    let steps = ((approx_len / 10.0) + (curvature / 6.0)).ceil() as usize;
    steps.clamp(4, 24)
  }

  fn append_curve_stamps(path_data: &mut String, from: &VectorPoint, to: &VectorPoint, width: f32) {
    let has_curve = from.handle_out.is_some() || to.handle_in.is_some();
    if !has_curve {
      append_line_stamps(path_data, from.x, from.y, to.x, to.y, width);
      return;
    }

    let p0 = (from.x, from.y);
    let p1 = from.handle_out.as_ref().map(|h| (h.x, h.y)).unwrap_or((from.x, from.y));
    let p2 = to.handle_in.as_ref().map(|h| (h.x, h.y)).unwrap_or((to.x, to.y));
    let p3 = (to.x, to.y);
    let steps = estimate_cubic_steps(p0, p1, p2, p3);
    let mut prev = p0;
    for i in 1..=steps {
      let t = (i as f32) / (steps as f32);
      let curr = cubic_point_xy(t, p0, p1, p2, p3);
      append_line_stamps(path_data, prev.0, prev.1, curr.0, curr.1, width);
      prev = curr;
    }
  }

  fn build_bezier_overlay_paths(objects: &[VectorObject]) -> BezierOverlayPaths {
    let mut overlay = BezierOverlayPaths::default();
    const CURVE_WIDTH: f32 = 2.8;
    const GUIDE_WIDTH: f32 = 1.6;
    const ANCHOR_SIZE: f32 = 3.0;
    const HANDLE_SIZE: f32 = 2.0;

    for object in objects {
      if object.object_type != "path" {
        continue;
      }
      for point in &object.points {
        append_square_marker_path(&mut overlay.anchor_marker_path_data, point.x, point.y, ANCHOR_SIZE);
        if let Some(handle_in) = &point.handle_in {
          append_line_stamps(&mut overlay.guide_path_data, point.x, point.y, handle_in.x, handle_in.y, GUIDE_WIDTH);
          append_square_marker_path(&mut overlay.handle_marker_path_data, handle_in.x, handle_in.y, HANDLE_SIZE);
        }
        if let Some(handle_out) = &point.handle_out {
          append_line_stamps(&mut overlay.guide_path_data, point.x, point.y, handle_out.x, handle_out.y, GUIDE_WIDTH);
          append_square_marker_path(&mut overlay.handle_marker_path_data, handle_out.x, handle_out.y, HANDLE_SIZE);
        }
      }
      if object.points.len() >= 2 {
        for idx in 1..object.points.len() {
          append_curve_stamps(&mut overlay.curve_path_data, &object.points[idx - 1], &object.points[idx], CURVE_WIDTH);
        }
        if object.closed {
          append_curve_stamps(
            &mut overlay.curve_path_data,
            &object.points[object.points.len() - 1],
            &object.points[0],
            CURVE_WIDTH,
          );
        }
      }
    }

    overlay
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
          path_data: build_square_marker_path(point.x, point.y, 3.0),
        });

        if let Some(handle_in) = &point.handle_in {
          markers.push(PointMarker {
            id: format!("{}-{}-in", object.id, point.id),
            path_data: build_square_marker_path(handle_in.x, handle_in.y, 2.0),
          });
        }

        if let Some(handle_out) = &point.handle_out {
          markers.push(PointMarker {
            id: format!("{}-{}-out", object.id, point.id),
            path_data: build_square_marker_path(handle_out.x, handle_out.y, 2.0),
          });
        }
      }
    }

    markers
  }

  fn set_pen_point_mirrored_handles(
    objects: &mut [VectorObject],
    path_id: &str,
    point_index: usize,
    mouse_x: f32,
    mouse_y: f32,
  ) -> bool {
    let Some(path_object) = objects.iter_mut().find(|object| object.id == path_id) else {
      return false;
    };
    if path_object.object_type != "path" || point_index >= path_object.points.len() {
      return false;
    }

    let point = &mut path_object.points[point_index];
    let dx = mouse_x - point.x;
    let dy = mouse_y - point.y;

    point.handle_in = Some(VectorHandle {
      x: point.x - dx,
      y: point.y - dy,
    });
    point.handle_out = Some(VectorHandle {
      x: point.x + dx,
      y: point.y + dy,
    });
    path_object.path_data = build_path_data(&path_object.points, path_object.closed);
    true
  }

  fn insert_pen_point(
    objects: &mut Vec<VectorObject>,
    current_path_id: &str,
    x: f32,
    y: f32,
    snapping_enabled: bool,
  ) -> PenInsertResult {
    if current_path_id.is_empty() {
      let path_number = next_path_number(objects);
      let new_id = format!("path-{}", path_number);
      let mut new_path = VectorObject {
        id: new_id.clone(),
        name: format!("Path {}", path_number),
        object_type: "path".to_string(),
        z_index: next_z_index(objects),
        closed: false,
        points: vec![VectorPoint {
          id: "pt-1".to_string(),
          x,
          y,
          handle_in: None,
          handle_out: None,
        }],
        path_data: String::new(),
      };
      new_path.path_data = build_path_data(&new_path.points, false);
      objects.push(new_path);
      return PenInsertResult::NewPathStarted {
        path_id: new_id,
        point_index: 0,
      };
    }

    let Some(path_object) = objects.iter_mut().find(|object| object.id == current_path_id) else {
      return PenInsertResult::NoOp;
    };

    let mut point_x = x;
    let mut point_y = y;
    let can_close = path_object.points.len() >= 2;
    if snapping_enabled && can_close {
      let first = &path_object.points[0];
      let dx = point_x - first.x;
      let dy = point_y - first.y;
      if (dx * dx) + (dy * dy) <= (SNAP_DISTANCE * SNAP_DISTANCE) {
        point_x = first.x;
        point_y = first.y;
        path_object.closed = true;
        path_object.path_data = build_path_data(&path_object.points, true);
        return PenInsertResult::PathClosed;
      }
    }

    let next_point_number = path_object.points.len() + 1;
    path_object.points.push(VectorPoint {
      id: format!("pt-{}", next_point_number),
      x: point_x,
      y: point_y,
      handle_in: None,
      handle_out: None,
    });
    path_object.path_data = build_path_data(&path_object.points, path_object.closed);

    PenInsertResult::PointAdded {
      path_id: path_object.id.clone(),
      point_index: path_object.points.len() - 1,
    }
  }

  fn distance(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let dx = x1 - x0;
    let dy = y1 - y0;
    ((dx * dx) + (dy * dy)).sqrt()
  }

  fn recompute_auto_handles(points: &mut [VectorPoint], closed: bool) {
    if points.len() < 2 {
      return;
    }

    for point in points.iter_mut() {
      point.handle_in = None;
      point.handle_out = None;
    }

    let point_count = points.len();
    for i in 0..point_count {
      let has_prev = i > 0 || closed;
      let has_next = i + 1 < point_count || closed;
      if !has_prev || !has_next {
        continue;
      }

      let prev_idx = if i == 0 { point_count - 1 } else { i - 1 };
      let next_idx = if i + 1 == point_count { 0 } else { i + 1 };

      let prev_x = points[prev_idx].x;
      let prev_y = points[prev_idx].y;
      let curr_x = points[i].x;
      let curr_y = points[i].y;
      let next_x = points[next_idx].x;
      let next_y = points[next_idx].y;

      let tangent_x = next_x - prev_x;
      let tangent_y = next_y - prev_y;
      let tangent_len = ((tangent_x * tangent_x) + (tangent_y * tangent_y)).sqrt();
      if tangent_len < 0.0001 {
        continue;
      }

      let dir_x = tangent_x / tangent_len;
      let dir_y = tangent_y / tangent_len;
      let in_len = (distance(prev_x, prev_y, curr_x, curr_y) * HANDLE_RATIO).min(MAX_HANDLE_LENGTH);
      let out_len = (distance(curr_x, curr_y, next_x, next_y) * HANDLE_RATIO).min(MAX_HANDLE_LENGTH);

      points[i].handle_in = Some(VectorHandle {
        x: curr_x - (dir_x * in_len),
        y: curr_y - (dir_y * in_len),
      });
      points[i].handle_out = Some(VectorHandle {
        x: curr_x + (dir_x * out_len),
        y: curr_y + (dir_y * out_len),
      });
    }
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
  let pen_down_pos = Rc::new(RefCell::new(None::<(f32, f32)>));
  let pen_drag_point = Rc::new(RefCell::new(None::<PenDragPoint>));

  let initial_objects = props.open_file.borrow().objects.clone();
  let initial_overlay = build_bezier_overlay_paths(&initial_objects);
  let (objects, set_objects) = crate::signals::create_signal::<Vec<VectorObject>>(initial_objects.clone());
  let (curve_overlay_path, set_curve_overlay_path) = crate::signals::create_signal::<String>(initial_overlay.curve_path_data);
  let (guide_overlay_path, set_guide_overlay_path) = crate::signals::create_signal::<String>(initial_overlay.guide_path_data);
  let (anchor_marker_path, set_anchor_marker_path) = crate::signals::create_signal::<String>(initial_overlay.anchor_marker_path_data);
  let (handle_marker_path, set_handle_marker_path) = crate::signals::create_signal::<String>(initial_overlay.handle_marker_path_data);

  let open_file_ref = props.open_file.clone();
  let commit_objects = Rc::new(move |next_objects: Vec<VectorObject>| {
    let next_overlay = build_bezier_overlay_paths(&next_objects);
    set_curve_overlay_path.set(next_overlay.curve_path_data);
    set_guide_overlay_path.set(next_overlay.guide_path_data);
    set_anchor_marker_path.set(next_overlay.anchor_marker_path_data);
    set_handle_marker_path.set(next_overlay.handle_marker_path_data);
    set_objects.set(next_objects.clone());
    open_file_ref.borrow_mut().objects = next_objects;
  });

  let engine_ref_down = engine.clone();
  let pen_down_pos_down = pen_down_pos.clone();
  let pen_drag_point_down = pen_drag_point.clone();
  let on_canvas_mouse_down = Rc::new(move |event: renderer_core::UiEvent| {
    if active_tool.get() != TOOL_BEZIER {
      *pen_down_pos_down.borrow_mut() = None;
      *pen_drag_point_down.borrow_mut() = None;
      return;
    }
    let (canvas_x, canvas_y) = engine_ref_down.borrow().get_node_final_position(event.current_target.id);
    let x = event.mouse_x - canvas_x;
    let y = event.mouse_y - canvas_y;
    *pen_down_pos_down.borrow_mut() = Some((x, y));
    *pen_drag_point_down.borrow_mut() = None;
  });

  let engine_ref_move = engine.clone();
  let pen_down_pos_move = pen_down_pos.clone();
  let pen_drag_point_move = pen_drag_point.clone();
  let commit_objects_move = commit_objects.clone();
  let on_canvas_mouse_move = Rc::new(move |event: renderer_core::UiEvent| {
    if active_tool.get() != TOOL_BEZIER {
      return;
    }
    let Some((down_x, down_y)) = *pen_down_pos_move.borrow() else {
      return;
    };

    let (canvas_x, canvas_y) = engine_ref_move.borrow().get_node_final_position(event.current_target.id);
    let x = event.mouse_x - canvas_x;
    let y = event.mouse_y - canvas_y;

    if pen_drag_point_move.borrow().is_none() {
      let dx = x - down_x;
      let dy = y - down_y;
      if (dx * dx) + (dy * dy) < PEN_DRAG_THRESHOLD_SQ {
        return;
      }

      let mut next_objects = objects.get();
      let current_path_id = active_path_id.get();
      match insert_pen_point(&mut next_objects, &current_path_id, down_x, down_y, snapping_enabled.get()) {
        PenInsertResult::NewPathStarted { path_id, point_index } => {
          let _ = set_pen_point_mirrored_handles(&mut next_objects, &path_id, point_index, x, y);
          commit_objects_move(next_objects);
          set_active_path_id.set(path_id.clone());
          *pen_drag_point_move.borrow_mut() = Some(PenDragPoint { path_id, point_index });
        }
        PenInsertResult::PointAdded { path_id, point_index } => {
          let _ = set_pen_point_mirrored_handles(&mut next_objects, &path_id, point_index, x, y);
          commit_objects_move(next_objects);
          *pen_drag_point_move.borrow_mut() = Some(PenDragPoint { path_id, point_index });
        }
        PenInsertResult::PathClosed => {
          commit_objects_move(next_objects);
          set_active_path_id.set(String::new());
          *pen_down_pos_move.borrow_mut() = None;
        }
        PenInsertResult::NoOp => {}
      }
      return;
    }

    let drag_state = pen_drag_point_move.borrow().clone().unwrap();
    let mut next_objects = objects.get();
    if set_pen_point_mirrored_handles(&mut next_objects, &drag_state.path_id, drag_state.point_index, x, y) {
      commit_objects_move(next_objects);
    }
  });

  let engine_ref_up = engine.clone();
  let pen_down_pos_up = pen_down_pos.clone();
  let pen_drag_point_up = pen_drag_point.clone();
  let commit_objects_up = commit_objects.clone();
  let on_canvas_mouse_up = Rc::new(move |event: renderer_core::UiEvent| {
    if active_tool.get() != TOOL_BEZIER {
      *pen_down_pos_up.borrow_mut() = None;
      *pen_drag_point_up.borrow_mut() = None;
      return;
    }
    let Some((down_x, down_y)) = *pen_down_pos_up.borrow() else {
      return;
    };

    let (canvas_x, canvas_y) = engine_ref_up.borrow().get_node_final_position(event.current_target.id);
    let x = event.mouse_x - canvas_x;
    let y = event.mouse_y - canvas_y;

    if pen_drag_point_up.borrow().is_some() {
      let drag_state = pen_drag_point_up.borrow().clone().unwrap();
      let mut next_objects = objects.get();
      if set_pen_point_mirrored_handles(&mut next_objects, &drag_state.path_id, drag_state.point_index, x, y) {
        commit_objects_up(next_objects);
      }
      *pen_drag_point_up.borrow_mut() = None;
      *pen_down_pos_up.borrow_mut() = None;
      return;
    }

    let mut next_objects = objects.get();
    let current_path_id = active_path_id.get();
    match insert_pen_point(&mut next_objects, &current_path_id, down_x, down_y, snapping_enabled.get()) {
      PenInsertResult::NewPathStarted { path_id, .. } => {
        commit_objects_up(next_objects);
        set_active_path_id.set(path_id);
      }
      PenInsertResult::PointAdded { .. } => {
        commit_objects_up(next_objects);
      }
      PenInsertResult::PathClosed => {
        commit_objects_up(next_objects);
        set_active_path_id.set(String::new());
      }
      PenInsertResult::NoOp => {}
    }
    *pen_down_pos_up.borrow_mut() = None;
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
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  fill: transparent;
  stroke: #ff2d2d;
  stroke-width: 2px;
}

.point-marker {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  fill: #ff2d2d;
  stroke: transparent;
  stroke-width: 0px;
}

.bezier-guide-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  fill: rgba(16, 145, 246, 0.55);
  stroke: transparent;
  stroke-width: 0px;
}

.bezier-curve-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  fill: #ff5a2a;
  stroke: transparent;
  stroke-width: 0px;
}

.bezier-handle-marker-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  fill: #1091f6;
  stroke: transparent;
  stroke-width: 0px;
}

.bezier-anchor-marker-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  fill: #ff2d2d;
  stroke: #111111;
  stroke-width: 1px;
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
