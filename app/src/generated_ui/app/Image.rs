#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use renderer_core::signals::{
    create_effect, create_memo, create_signal, ReadSignal, ToBool, ToReactiveString,
};
use renderer_core::ui::{div, img, input, mount_if, mount_list, text, Element};
use renderer_core::FlexEngine;
use std::cell::RefCell;
use std::rc::Rc;
#[allow(non_snake_case)]
pub mod SmartShapeMenuItem;
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
        let in_len =
            (distance(prev_x, prev_y, curr_x, curr_y) * HANDLE_RATIO).min(MAX_HANDLE_LENGTH);
        let out_len =
            (distance(curr_x, curr_y, next_x, next_y) * HANDLE_RATIO).min(MAX_HANDLE_LENGTH);
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
    objects
        .iter()
        .filter(|object| object.object_type == "path")
        .count()
        + 1
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
pub type SlotRenderFn = Rc<dyn Fn(Rc<RefCell<FlexEngine>>, u32, SlotScope)>;
#[derive(Clone, Default)]
pub struct SlotScope {
    values: std::collections::HashMap<String, String>,
}
impl SlotScope {
    pub fn new(values: std::collections::HashMap<String, String>) -> Self {
        Self { values }
    }
    pub fn get(&self, key: &str) -> String {
        self.values.get(key).cloned().unwrap_or_default()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
}
impl std::ops::Index<&str> for SlotScope {
    type Output = str;
    fn index(&self, key: &str) -> &Self::Output {
        self.values
            .get(key)
            .map(|value| value.as_str())
            .unwrap_or("")
    }
}
#[derive(Clone, Default)]
pub struct Slots {
    pub default: Option<SlotRenderFn>,
    pub named: std::collections::HashMap<String, SlotRenderFn>,
}
impl Slots {
    pub fn get(&self, name: &str) -> Option<SlotRenderFn> {
        self.named.get(name).cloned()
    }
}
pub fn build(engine: Rc<RefCell<FlexEngine>>, parent: Option<u32>, props: Props) -> u32 {
    build_with_slots(engine, parent, props, Slots::default())
}
#[allow(unused_variables)]
pub fn build_with_slots(
    engine: Rc<RefCell<FlexEngine>>,
    parent: Option<u32>,
    props: Props,
    slots: Slots,
) -> u32 {
    register_styles(engine.clone());
    let (file_menu_open, set_file_menu_open) = crate::signals::create_signal(false);
    let (active_tool, set_active_tool) = crate::signals::create_signal(TOOL_SELECT.to_string());
    let (snapping_enabled, set_snapping_enabled) = crate::signals::create_signal(true);
    let (active_path_id, set_active_path_id) = crate::signals::create_signal(String::new());
    let initial_objects = props.open_file.borrow().objects.clone();
    let (objects, set_objects) =
        crate::signals::create_signal::<Vec<VectorObject>>(initial_objects.clone());
    let (point_markers, set_point_markers) =
        crate::signals::create_signal::<Vec<PointMarker>>(build_point_markers(&initial_objects));
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
        let (canvas_x, canvas_y) = engine_ref
            .borrow()
            .get_node_final_position(event.current_target.id);
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
                    handle_in: None,
                    handle_out: None,
                }],
                path_data: String::new(),
            };
            new_path.path_data = build_path_data(&new_path.points, false);
            next_objects.push(new_path);
            commit_objects(next_objects);
            set_active_path_id.set(new_id);
            return;
        }
        if let Some(path_object) = next_objects
            .iter_mut()
            .find(|object| object.id == current_path_id)
        {
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
                recompute_auto_handles(&mut path_object.points, true);
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
                handle_in: None,
                handle_out: None,
            });
            recompute_auto_handles(&mut path_object.points, false);
            path_object.path_data = build_path_data(&path_object.points, false);
            commit_objects(next_objects);
        }
    });
    let root_id = {
        let node_1 = div()
            .class("editor-container")
            .build(engine.clone(), parent);
        {
            let node_2 = div().class("sidebar").build(engine.clone(), Some(node_1));
            {
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_2,
                        create_memo(move || (active_tool == TOOL_SELECT).to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            let __mount_if_parent = node_2;
                            let node_3 = div()
                                .class("tool-icon active")
                                .on_click(move |event: renderer_core::UiEvent| {
                                    set_active_tool.set(TOOL_SELECT.to_string());
                                    set_active_path_id.set(String::new());
                                })
                                .build(engine.clone(), Some(__mount_if_parent));
                            {
                                let node_4 = img()
                                    .image("asset://phosphor/selection.svg")
                                    .class("icon-img")
                                    .build(engine.clone(), Some(node_3));
                                {}
                                node_4;
                            }
                            node_3
                        },
                    );
                    0
                };
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_2,
                        create_memo(move || (active_tool != TOOL_SELECT).to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            let __mount_if_parent = node_2;
                            let node_5 = div()
                                .class("tool-icon")
                                .on_click(move |event: renderer_core::UiEvent| {
                                    set_active_tool.set(TOOL_SELECT.to_string());
                                    set_active_path_id.set(String::new());
                                })
                                .build(engine.clone(), Some(__mount_if_parent));
                            {
                                let node_6 = img()
                                    .image("asset://phosphor/selection.svg")
                                    .class("icon-img")
                                    .build(engine.clone(), Some(node_5));
                                {}
                                node_6;
                            }
                            node_5
                        },
                    );
                    0
                };
                let node_7 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_8 = img()
                        .image("asset://phosphor/cursor.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_7));
                    {}
                    node_8;
                }
                node_7;
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_2,
                        create_memo(move || (active_tool == TOOL_BEZIER).to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            let __mount_if_parent = node_2;
                            let node_9 = div()
                                .class("tool-icon active")
                                .on_click(move |event: renderer_core::UiEvent| {
                                    set_active_tool.set(TOOL_BEZIER.to_string());
                                })
                                .build(engine.clone(), Some(__mount_if_parent));
                            {
                                let node_10 = img()
                                    .image("asset://phosphor/bezier-curve.svg")
                                    .class("icon-img")
                                    .build(engine.clone(), Some(node_9));
                                {}
                                node_10;
                            }
                            node_9
                        },
                    );
                    0
                };
                {
                    let engine_c = engine.clone();
                    mount_if(
                        engine.clone(),
                        node_2,
                        create_memo(move || (active_tool != TOOL_BEZIER).to_bool()),
                        move || {
                            let engine = engine_c.clone();
                            let __mount_if_parent = node_2;
                            let node_11 = div()
                                .class("tool-icon")
                                .on_click(move |event: renderer_core::UiEvent| {
                                    set_active_tool.set(TOOL_BEZIER.to_string());
                                })
                                .build(engine.clone(), Some(__mount_if_parent));
                            {
                                let node_12 = img()
                                    .image("asset://phosphor/bezier-curve.svg")
                                    .class("icon-img")
                                    .build(engine.clone(), Some(node_11));
                                {}
                                node_12;
                            }
                            node_11
                        },
                    );
                    0
                };
                self::SmartShapeMenuItem::build_with_slots(
                    engine.clone(),
                    Some(node_2),
                    self::SmartShapeMenuItem::Props {},
                    {
                        let mut __named_slots: std::collections::HashMap<
                            String,
                            self::SmartShapeMenuItem::SlotRenderFn,
                        > = std::collections::HashMap::new();
                        __named_slots . insert ("flyout" . to_string () , Rc :: new (move | engine : Rc < RefCell < FlexEngine >> , slot_parent : u32 , slot_scope : self :: SmartShapeMenuItem :: SlotScope | { let __slot_parent = slot_parent ; { let engine_c = engine . clone () ; mount_if (engine . clone () , __slot_parent , create_memo (move || (active_tool == TOOL_SHAPE_CIRCLE) . to_bool ()) , move || { let engine = engine_c . clone () ; let __mount_if_parent = __slot_parent ; let node_17 = div () . class ("smart-shape-flyout-tool-icon active") . on_click (move | event : renderer_core :: UiEvent | { set_active_path_id . set (String :: new ()) ; set_active_tool . set (TOOL_SHAPE_CIRCLE . to_string ()) ; renderer_core :: log ("tool: shape-circle") ; }) . build (engine . clone () , Some (__mount_if_parent)) ; { let node_18 = img () . image ("asset://phosphor/circle.svg") . class ("smart-shape-icon-img") . build (engine . clone () , Some (node_17)) ; { } node_18 ; } node_17 }) ; 0 } ; { let engine_c = engine . clone () ; mount_if (engine . clone () , __slot_parent , create_memo (move || (active_tool != TOOL_SHAPE_CIRCLE) . to_bool ()) , move || { let engine = engine_c . clone () ; let __mount_if_parent = __slot_parent ; let node_19 = div () . class ("smart-shape-flyout-tool-icon") . on_click (move | event : renderer_core :: UiEvent | { set_active_path_id . set (String :: new ()) ; set_active_tool . set (TOOL_SHAPE_CIRCLE . to_string ()) ; renderer_core :: log ("tool: shape-circle") ; }) . build (engine . clone () , Some (__mount_if_parent)) ; { let node_20 = img () . image ("asset://phosphor/circle.svg") . class ("smart-shape-icon-img") . build (engine . clone () , Some (node_19)) ; { } node_20 ; } node_19 }) ; 0 } ; { let engine_c = engine . clone () ; mount_if (engine . clone () , __slot_parent , create_memo (move || (active_tool == TOOL_SHAPE_LINE) . to_bool ()) , move || { let engine = engine_c . clone () ; let __mount_if_parent = __slot_parent ; let node_21 = div () . class ("smart-shape-flyout-tool-icon active") . on_click (move | event : renderer_core :: UiEvent | { set_active_path_id . set (String :: new ()) ; set_active_tool . set (TOOL_SHAPE_LINE . to_string ()) ; renderer_core :: log ("tool: shape-line") ; }) . build (engine . clone () , Some (__mount_if_parent)) ; { let node_22 = img () . image ("asset://phosphor/line.svg") . class ("smart-shape-icon-img") . build (engine . clone () , Some (node_21)) ; { } node_22 ; } node_21 }) ; 0 } ; { let engine_c = engine . clone () ; mount_if (engine . clone () , __slot_parent , create_memo (move || (active_tool != TOOL_SHAPE_LINE) . to_bool ()) , move || { let engine = engine_c . clone () ; let __mount_if_parent = __slot_parent ; let node_23 = div () . class ("smart-shape-flyout-tool-icon") . on_click (move | event : renderer_core :: UiEvent | { set_active_path_id . set (String :: new ()) ; set_active_tool . set (TOOL_SHAPE_LINE . to_string ()) ; renderer_core :: log ("tool: shape-line") ; }) . build (engine . clone () , Some (__mount_if_parent)) ; { let node_24 = img () . image ("asset://phosphor/line.svg") . class ("smart-shape-icon-img") . build (engine . clone () , Some (node_23)) ; { } node_24 ; } node_23 }) ; 0 } ; }) as self :: SmartShapeMenuItem :: SlotRenderFn) ;
                        self :: SmartShapeMenuItem :: Slots { default : Some (Rc :: new (move | engine : Rc < RefCell < FlexEngine >> , slot_parent : u32 , slot_scope : self :: SmartShapeMenuItem :: SlotScope | { let __slot_parent = slot_parent ; { let engine_c = engine . clone () ; mount_if (engine . clone () , __slot_parent , create_memo (move || (active_tool == TOOL_SHAPE_RECT) . to_bool ()) , move || { let engine = engine_c . clone () ; let __mount_if_parent = __slot_parent ; let node_13 = div () . class ("smart-shape-trigger-icon active") . on_click (move | event : renderer_core :: UiEvent | { set_active_path_id . set (String :: new ()) ; set_active_tool . set (TOOL_SHAPE_RECT . to_string ()) ; renderer_core :: log ("tool: shape-rect") ; }) . build (engine . clone () , Some (__mount_if_parent)) ; { let node_14 = img () . image ("asset://phosphor/square.svg") . class ("smart-shape-icon-img") . build (engine . clone () , Some (node_13)) ; { } node_14 ; } node_13 }) ; 0 } ; { let engine_c = engine . clone () ; mount_if (engine . clone () , __slot_parent , create_memo (move || (active_tool != TOOL_SHAPE_RECT) . to_bool ()) , move || { let engine = engine_c . clone () ; let __mount_if_parent = __slot_parent ; let node_15 = div () . class ("smart-shape-trigger-icon") . on_click (move | event : renderer_core :: UiEvent | { set_active_path_id . set (String :: new ()) ; set_active_tool . set (TOOL_SHAPE_RECT . to_string ()) ; renderer_core :: log ("tool: shape-rect") ; }) . build (engine . clone () , Some (__mount_if_parent)) ; { let node_16 = img () . image ("asset://phosphor/square.svg") . class ("smart-shape-icon-img") . build (engine . clone () , Some (node_15)) ; { } node_16 ; } node_15 }) ; 0 } ; }) as self :: SmartShapeMenuItem :: SlotRenderFn) , named : __named_slots , }
                    },
                );
                let node_25 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_26 = img()
                        .image("asset://phosphor/hand-grabbing.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_25));
                    {}
                    node_26;
                }
                node_25;
                let node_27 = div().class("spacer").build(engine.clone(), Some(node_2));
                {}
                node_27;
                let node_28 = div().class("tool-icon").build(engine.clone(), Some(node_2));
                {
                    let node_29 = img()
                        .image("asset://phosphor/settings.svg")
                        .class("icon-img")
                        .build(engine.clone(), Some(node_28));
                    {}
                    node_29;
                }
                node_28;
            }
            node_2;
            let node_30 = div()
                .class("main-content")
                .build(engine.clone(), Some(node_1));
            {
                let node_31 = div().class("top-bar").build(engine.clone(), Some(node_30));
                {
                    let node_32 = div()
                        .class("file-menu-container")
                        .build(engine.clone(), Some(node_31));
                    {
                        let node_33 = div()
                            .class("menu-item file-menu-trigger")
                            .on_mouse_enter(move |event: renderer_core::UiEvent| {
                                set_file_menu_open.set(true);
                            })
                            .on_mouse_leave(move |event: renderer_core::UiEvent| {
                                set_file_menu_open.set(false);
                            })
                            .build(engine.clone(), Some(node_32));
                        {
                            div().text("File").build(engine.clone(), Some(node_33));
                        }
                        node_33;
                        let node_34 = div()
                            .class("file-menu-dropdown")
                            .on_mouse_enter(move |event: renderer_core::UiEvent| {
                                set_file_menu_open.set(true);
                            })
                            .on_mouse_leave(move |event: renderer_core::UiEvent| {
                                set_file_menu_open.set(false);
                            })
                            .build(engine.clone(), Some(node_32));
                        {
                            let node_35 = div()
                                .class("file-menu-command")
                                .on_click(move |event: renderer_core::UiEvent| {
                                    renderer_core::log("menu: command new file");
                                })
                                .build(engine.clone(), Some(node_34));
                            {
                                div().text("New File").build(engine.clone(), Some(node_35));
                            }
                            node_35;
                            let node_36 = div()
                                .class("file-menu-command")
                                .on_click(move |event: renderer_core::UiEvent| {
                                    renderer_core::log("menu: command open");
                                })
                                .build(engine.clone(), Some(node_34));
                            {
                                div().text("Open...").build(engine.clone(), Some(node_36));
                            }
                            node_36;
                            let node_37 = div()
                                .class("file-menu-command")
                                .on_click(move |event: renderer_core::UiEvent| {
                                    renderer_core::log("menu: command save");
                                })
                                .build(engine.clone(), Some(node_34));
                            {
                                div().text("Save").build(engine.clone(), Some(node_37));
                            }
                            node_37;
                            let node_38 = div()
                                .class("file-menu-command")
                                .on_click(move |event: renderer_core::UiEvent| {
                                    renderer_core::log("menu: command export");
                                })
                                .build(engine.clone(), Some(node_34));
                            {
                                div().text("Export").build(engine.clone(), Some(node_38));
                            }
                            node_38;
                        }
                        create_effect({
                            let engine = engine.clone();
                            move || {
                                let visible = (file_menu_open == true).to_bool();
                                engine.borrow_mut().set_node_visible(node_34, visible);
                            }
                        });
                        node_34;
                    }
                    node_32;
                    let node_39 = div()
                        .class("menu-item")
                        .build(engine.clone(), Some(node_31));
                    {
                        div().text("Edit").build(engine.clone(), Some(node_39));
                    }
                    node_39;
                    let node_40 = div()
                        .class("menu-item")
                        .build(engine.clone(), Some(node_31));
                    {
                        div().text("View").build(engine.clone(), Some(node_40));
                    }
                    node_40;
                    let node_41 = div()
                        .class("menu-item")
                        .build(engine.clone(), Some(node_31));
                    {
                        div().text("Object").build(engine.clone(), Some(node_41));
                    }
                    node_41;
                    let node_42 = div().class("spacer").build(engine.clone(), Some(node_31));
                    {}
                    node_42;
                    let node_43 = div()
                        .class("project-title")
                        .build(engine.clone(), Some(node_31));
                    {
                        div()
                            .value(create_memo({
                                let val = props.open_file.borrow().path.clone().clone();
                                move || val.to_reactive_string()
                            }))
                            .build(engine.clone(), Some(node_43));
                    }
                    node_43;
                    let node_44 = div().class("spacer").build(engine.clone(), Some(node_31));
                    {}
                    node_44;
                    let node_45 = div()
                        .class("user-profile")
                        .build(engine.clone(), Some(node_31));
                    {
                        div().text("HB").build(engine.clone(), Some(node_45));
                    }
                    node_45;
                }
                node_31;
                let node_46 = div()
                    .class("context-bar")
                    .build(engine.clone(), Some(node_30));
                {
                    let node_47 = div()
                        .class("context-tools")
                        .build(engine.clone(), Some(node_46));
                    {
                        let node_48 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_47));
                        {
                            let node_49 = img()
                                .image("asset://phosphor/undo.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_48));
                            {}
                            node_49;
                        }
                        node_48;
                        let node_50 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_47));
                        {
                            let node_51 = img()
                                .image("asset://phosphor/redo.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_50));
                            {}
                            node_51;
                        }
                        node_50;
                        let node_52 = div().class("divider").build(engine.clone(), Some(node_47));
                        {}
                        node_52;
                        let node_53 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_47));
                        {
                            let node_54 = img()
                                .image("asset://phosphor/copy.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_53));
                            {}
                            node_54;
                        }
                        node_53;
                        let node_55 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_47));
                        {
                            let node_56 = img()
                                .image("asset://phosphor/paste.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_55));
                            {}
                            node_56;
                        }
                        node_55;
                        let node_57 = div()
                            .class("tool-icon-small")
                            .build(engine.clone(), Some(node_47));
                        {
                            let node_58 = img()
                                .image("asset://phosphor/delete.svg")
                                .class("icon-img-small")
                                .build(engine.clone(), Some(node_57));
                            {}
                            node_58;
                        }
                        node_57;
                    }
                    node_47;
                    let node_59 = div().class("spacer").build(engine.clone(), Some(node_46));
                    {}
                    node_59;
                    let node_60 = div()
                        .class("snap-tools")
                        .build(engine.clone(), Some(node_46));
                    {
                        {
                            let engine_c = engine.clone();
                            mount_if(
                                engine.clone(),
                                node_60,
                                create_memo(move || (snapping_enabled == true).to_bool()),
                                move || {
                                    let engine = engine_c.clone();
                                    let __mount_if_parent = node_60;
                                    let node_61 = div()
                                        .class("tool-icon-small active")
                                        .on_click(move |event: renderer_core::UiEvent| {
                                            set_snapping_enabled.set(false);
                                        })
                                        .build(engine.clone(), Some(__mount_if_parent));
                                    {
                                        let node_62 = img()
                                            .image("asset://phosphor/snapping.svg")
                                            .class("icon-img-small")
                                            .build(engine.clone(), Some(node_61));
                                        {}
                                        node_62;
                                    }
                                    node_61
                                },
                            );
                            0
                        };
                        {
                            let engine_c = engine.clone();
                            mount_if(
                                engine.clone(),
                                node_60,
                                create_memo(move || (snapping_enabled != true).to_bool()),
                                move || {
                                    let engine = engine_c.clone();
                                    let __mount_if_parent = node_60;
                                    let node_63 = div()
                                        .class("tool-icon-small")
                                        .on_click(move |event: renderer_core::UiEvent| {
                                            set_snapping_enabled.set(true);
                                        })
                                        .build(engine.clone(), Some(__mount_if_parent));
                                    {
                                        let node_64 = img()
                                            .image("asset://phosphor/snapping.svg")
                                            .class("icon-img-small")
                                            .build(engine.clone(), Some(node_63));
                                        {}
                                        node_64;
                                    }
                                    node_63
                                },
                            );
                            0
                        };
                        let node_65 = div()
                            .class("label-small")
                            .build(engine.clone(), Some(node_60));
                        {
                            div().text("Snapping").build(engine.clone(), Some(node_65));
                        }
                        node_65;
                    }
                    node_60;
                }
                node_46;
                let node_66 = div()
                    .class("editor-body")
                    .build(engine.clone(), Some(node_30));
                {
                    let node_67 = div()
                        .class("canvas-area")
                        .build(engine.clone(), Some(node_66));
                    {
                        let node_68 = div()
                            .class("canvas-mock")
                            .on_click(move |event: renderer_core::UiEvent| {
                                (on_canvas_click)(event);
                            })
                            .build(engine.clone(), Some(node_67));
                        {
                            let node_69 = div()
                                .class("path-layer")
                                .build(engine.clone(), Some(node_68));
                            {
                                {
                                    mount_list(
                                        engine.clone(),
                                        node_69,
                                        objects,
                                        |item| item.id.clone(),
                                        move |obj| {
                                            div().class("path-shape").bind_path(create_memo({
                                                let val = obj.path_data.clone().clone();
                                                move || val.to_reactive_string()
                                            }))
                                        },
                                    );
                                    0
                                };
                                {
                                    mount_list(
                                        engine.clone(),
                                        node_69,
                                        point_markers,
                                        |item| item.id.clone(),
                                        move |marker| {
                                            div().class("point-marker").bind_path(create_memo({
                                                let val = marker.path_data.clone().clone();
                                                move || val.to_reactive_string()
                                            }))
                                        },
                                    );
                                    0
                                };
                            }
                            node_69;
                        }
                        node_68;
                    }
                    node_67;
                    let node_72 = div()
                        .class("right-panel")
                        .build(engine.clone(), Some(node_66));
                    {
                        let node_73 = div()
                            .class("panel-header")
                            .build(engine.clone(), Some(node_72));
                        {
                            let node_74 = img()
                                .image("asset://phosphor/layers.svg")
                                .class("icon-img-mini")
                                .build(engine.clone(), Some(node_73));
                            {}
                            node_74;
                            let node_75 = div()
                                .class("panel-title")
                                .build(engine.clone(), Some(node_73));
                            {
                                div().text("Objects").build(engine.clone(), Some(node_75));
                            }
                            node_75;
                        }
                        node_73;
                        let node_76 = div()
                            .class("layer-list")
                            .build(engine.clone(), Some(node_72));
                        {
                            {
                                mount_list(
                                    engine.clone(),
                                    node_76,
                                    objects,
                                    |item| item.id.clone(),
                                    move |obj| {
                                        div()
                                            .class("layer-item")
                                            .child(
                                                img()
                                                    .image("asset://phosphor/pencil.svg")
                                                    .class("icon-img-mini"),
                                            )
                                            .child(div().class("layer-name").child(div().value(
                                                create_memo({
                                                    let val = format!(
                                                        "{} (z={:.0})",
                                                        obj.name, obj.z_index
                                                    )
                                                    .clone();
                                                    move || val.to_reactive_string()
                                                }),
                                            )))
                                    },
                                );
                                0
                            };
                        }
                        node_76;
                    }
                    node_72;
                }
                node_66;
            }
            node_30;
        }
        node_1
    };
    root_id
}
fn register_styles(engine: Rc<RefCell<FlexEngine>>) {
    #[allow(unused_mut)]
    let mut e = engine.borrow_mut();
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.07058824f32, 0.07058824f32, 0.07058824f32, 1f32),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8784314f32, 0.8784314f32, 0.8784314f32, 1f32),
        );
        e.add_style_rule(".editor-container".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(64f32));
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".sidebar".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(44f32));
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(44f32));
        e.add_style_rule(".tool-icon".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        e.add_style_rule(".tool-icon:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.33333334f32, 0.33333334f32, 0.33333334f32, 1f32),
        );
        e.add_style_rule(".tool-icon.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(28f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(28f32));
        e.add_style_rule(".icon-img".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".main-content".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(44f32));
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(16f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".top-bar".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".file-menu-container".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.73333335f32, 0.73333335f32, 0.73333335f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(13f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        e.add_style_rule(".menu-item".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "z-index".to_string(),
            renderer_core::StyleValue::Px(5000f32),
        );
        e.add_style_rule(".file-menu-trigger".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        e.add_style_rule(".menu-item:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14117648f32, 0.14117648f32, 0.14117648f32, 1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.23921569f32, 0.23921569f32, 0.23921569f32, 1f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.23921569f32, 0.23921569f32, 0.23921569f32, 1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "z-index".to_string(),
            renderer_core::StyleValue::Px(9000f32),
        );
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.45f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.23921569f32, 0.23921569f32, 0.23921569f32, 1f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "top".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(24f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "min-width".to_string(),
            renderer_core::StyleValue::Px(170f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.23921569f32, 0.23921569f32, 0.23921569f32, 1f32),
        );
        e.add_style_rule(".file-menu-dropdown".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8156863f32, 0.8156863f32, 0.8156863f32, 1f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(30f32));
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        e.add_style_rule(".file-menu-command".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.22745098f32, 0.22745098f32, 0.22745098f32, 1f32),
        );
        e.add_style_rule(".file-menu-command:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(13f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.53333336f32, 0.53333336f32, 0.53333336f32, 1f32),
        );
        e.add_style_rule(".project-title".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(48f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14509805f32, 0.14509805f32, 0.14509805f32, 1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        e.add_style_rule(".context-bar".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".context-tools".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(32f32));
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(32f32));
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        e.add_style_rule(".tool-icon-small".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.21960784f32, 0.21960784f32, 0.21960784f32, 1f32),
        );
        e.add_style_rule(".tool-icon-small:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        e.add_style_rule(".tool-icon-small.active".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(18f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(18f32));
        e.add_style_rule(".icon-img-small".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "margin-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(1f32));
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(20f32));
        e.add_style_rule(".divider".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "border-top-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(4f32),
        );
        decls.insert(
            "border-bottom-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-right-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-bottom".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "border-color-top".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        decls.insert(
            "border-color-right".to_string(),
            renderer_core::StyleValue::Color(0.26666668f32, 0.26666668f32, 0.26666668f32, 1f32),
        );
        e.add_style_rule(".snap-tools".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(11f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8f32, 0.8f32, 0.8f32, 1f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(6f32),
        );
        e.add_style_rule(".label-small".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        e.add_style_rule(".editor-body".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "justify-content".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(40f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.07058824f32, 0.07058824f32, 0.07058824f32, 1f32),
        );
        e.add_style_rule(".canvas-area".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "box-shadow-color".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0.5f32),
        );
        decls.insert(
            "box-shadow-blur".to_string(),
            renderer_core::StyleValue::Px(30f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(400f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(600f32));
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(1f32, 1f32, 1f32, 1f32),
        );
        decls.insert(
            "box-shadow-v-offset".to_string(),
            renderer_core::StyleValue::Px(10f32),
        );
        decls.insert(
            "box-shadow-spread".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("relative".to_string()),
        );
        decls.insert(
            "box-shadow-h-offset".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        e.add_style_rule(".canvas-mock".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("left".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert("top".to_string(), renderer_core::StyleValue::Px(0f32));
        decls.insert(
            "height".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        decls.insert(
            "position".to_string(),
            renderer_core::StyleValue::Ident("absolute".to_string()),
        );
        decls.insert(
            "width".to_string(),
            renderer_core::StyleValue::Percent(100f32),
        );
        e.add_style_rule(".path-layer".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "stroke".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "fill".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        decls.insert(
            "stroke-width".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        e.add_style_rule(".path-shape".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "stroke-width".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "fill".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.1764706f32, 0.1764706f32, 1f32),
        );
        decls.insert(
            "stroke".to_string(),
            renderer_core::StyleValue::Color(0f32, 0f32, 0f32, 0f32),
        );
        e.add_style_rule(".point-marker".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(240f32));
        decls.insert(
            "border-color-left".to_string(),
            renderer_core::StyleValue::Color(0.2f32, 0.2f32, 0.2f32, 1f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.11764706f32, 0.11764706f32, 0.11764706f32, 1f32),
        );
        decls.insert(
            "border-left-width".to_string(),
            renderer_core::StyleValue::Px(1f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        e.add_style_rule(".right-panel".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(40f32));
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.14509805f32, 0.14509805f32, 0.14509805f32, 1f32),
        );
        e.add_style_rule(".panel-header".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(1f32, 0.64705884f32, 0f32, 1f32),
        );
        decls.insert(
            "text-transform".to_string(),
            renderer_core::StyleValue::Ident("uppercase".to_string()),
        );
        decls.insert(
            "letter-spacing".to_string(),
            renderer_core::StyleValue::Em(0.05f32),
        );
        decls.insert(
            "font-weight".to_string(),
            renderer_core::StyleValue::Px(600f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(11f32),
        );
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        e.add_style_rule(".panel-title".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("column".to_string()),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        e.add_style_rule(".layer-list".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "padding-top".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert(
            "padding-right".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "padding-bottom".to_string(),
            renderer_core::StyleValue::Px(0f32),
        );
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(32f32));
        decls.insert(
            "align-items".to_string(),
            renderer_core::StyleValue::Ident("center".to_string()),
        );
        decls.insert(
            "margin-bottom".to_string(),
            renderer_core::StyleValue::Px(2f32),
        );
        decls.insert(
            "flex-direction".to_string(),
            renderer_core::StyleValue::Ident("row".to_string()),
        );
        e.add_style_rule(".layer-item".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "background-color".to_string(),
            renderer_core::StyleValue::Color(0.16470589f32, 0.16470589f32, 0.16470589f32, 1f32),
        );
        e.add_style_rule(".layer-item:hover".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert(
            "margin-left".to_string(),
            renderer_core::StyleValue::Px(8f32),
        );
        decls.insert(
            "color".to_string(),
            renderer_core::StyleValue::Color(0.8f32, 0.8f32, 0.8f32, 1f32),
        );
        decls.insert(
            "font-size".to_string(),
            renderer_core::StyleValue::Px(12f32),
        );
        e.add_style_rule(".layer-name".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        decls.insert("height".to_string(), renderer_core::StyleValue::Px(14f32));
        decls.insert("width".to_string(), renderer_core::StyleValue::Px(14f32));
        e.add_style_rule(".icon-img-mini".to_string(), decls);
    }
    {
        let mut decls = std::collections::HashMap::new();
        e.add_style_rule(".spacer".to_string(), decls);
    }
}
