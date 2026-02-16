#[cfg(test)]
mod tests {
    use crate::signals::{create_effect, create_root, create_signal};
    use crate::ui::div;
    use crate::{FlexEngine, StyleValue};

    #[test]
    fn test_basic_signal() {
        let (count, set_count) = create_signal(0);
        assert_eq!(count.get(), 0);

        set_count.set(5);
        assert_eq!(count.get(), 5);
    }

    #[test]
    fn test_effect() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let (count, set_count) = create_signal(0);
        let result = Rc::new(RefCell::new(0));
        let result_clone = result.clone();

        create_effect(move || {
            *result_clone.borrow_mut() = count.get() * 2;
        });

        assert_eq!(*result.borrow(), 0);
        set_count.set(10);
        assert_eq!(*result.borrow(), 20);
    }

    #[test]
    fn test_builder_structure() {
        use crate::ui::{div, text};
        let _layout = div()
            .width(100.0)
            .row()
            .child(text("Hello"))
            .child(div().col());
        assert!(true);
    }

    #[test]
    fn test_reactive_flags() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let engine = Rc::new(RefCell::new(FlexEngine::new()));
        let (visible, set_visible) = create_signal(1u32);

        let node_id = div().bind_flags(visible).build(engine.clone(), None);

        engine.borrow_mut().render();

        unsafe {
            let engine_borrow = engine.borrow();
            let nodes = engine_borrow.get_nodes_ptr();
            let flags = (*nodes.add(node_id as usize)).flags;
            assert_eq!(flags, 1);
        }

        set_visible.set(0u32);
        engine.borrow_mut().render();

        unsafe {
            let engine_borrow = engine.borrow();
            let nodes = engine_borrow.get_nodes_ptr();
            let flags = (*nodes.add(node_id as usize)).flags;
            assert_eq!(flags, 0);
        }
    }

    #[test]
    fn test_dynamic_list() {
        use crate::ui::mount_list;
        use std::cell::RefCell;
        use std::rc::Rc;

        let engine = Rc::new(RefCell::new(FlexEngine::new()));
        let (items, set_items) = create_signal(vec!["A".to_string(), "B".to_string()]);

        let root_id = {
            let mut e = engine.borrow_mut();
            e.add_node(100.0)
        };

        mount_list(
            engine.clone(),
            root_id,
            items,
            |name| name.clone(),
            |name| div().text(&name),
        );

        engine.borrow_mut().render();
        assert_eq!(engine.borrow().get_node_count(), 4);

        set_items.set(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
        engine.borrow_mut().render();
        assert_eq!(engine.borrow().get_node_count(), 5);

        set_items.set(vec!["X".to_string()]);
        engine.borrow_mut().render();
        assert_eq!(engine.borrow().get_node_count(), 3);
    }
    #[test]
    fn test_atlas_memory_management() {
        use crate::texture_atlas::{CacheKey, TextureAtlas};
        use std::rc::Rc;

        // 1024x1024 atlas
        let mut atlas = TextureAtlas::new(1024, 1024);
        let key = CacheKey {
            id: "test".to_string(),
            width: 100,
            height: 100,
            fill_color: 0,
            stroke_color: 0,
            stroke_width: 0,
            text_color: 0xFFFFFFFF,
        };
        // Create dummy pixels
        let pixels = vec![0u8; (100 * 100 * 4) as usize];

        // 1. Allocate
        let handle = atlas
            .allocate(key.clone(), pixels)
            .expect("Allocation failed");
        assert_eq!(Rc::strong_count(&handle), 1);

        // 2. Clone Handle (Ref Count 2)
        let handle2 = handle.clone();
        assert_eq!(Rc::strong_count(&handle), 2);

        // 3. Drop one
        drop(handle);
        // Handle still alive via handle2, so NOT deallocated
        atlas.process_deletions();

        // 4. Drop last one
        // This should push ID to the queue
        drop(handle2);

        // 5. Process Deletions
        // This should trigger the DeallocationQueue and free the space in allocator
        atlas.process_deletions();

        // 6. Check if cache cleared (Weak ref dead)
        // Since we dropped all Rcs, the Weak in cache should be upgradable to None
        let h3 = atlas.get_handle(&key);
        assert!(h3.is_none(), "Cache should allow handle to die");
    }

    #[test]
    fn test_style_rule_deduplication() {
        let mut engine = FlexEngine::new();
        let mut decls = std::collections::HashMap::new();
        decls.insert("width".to_string(), StyleValue::Px(100.0));

        engine.add_style_rule(".test".to_string(), decls.clone());
        assert_eq!(engine.stylesheet.rules.len(), 1);

        // Add same selector again
        engine.add_style_rule(".test".to_string(), decls);
        assert_eq!(
            engine.stylesheet.rules.len(),
            1,
            "Should replace existing rule, not append"
        );
    }

    #[test]
    #[should_panic(expected = "Unknown CSS property")]
    fn test_unknown_inline_style_property_panics() {
        let engine = std::rc::Rc::new(std::cell::RefCell::new(FlexEngine::new()));
        div()
            .style("unknown-prop-for-test", StyleValue::Px(1.0))
            .build(engine.clone(), None);
        engine.borrow_mut().render();
    }

    #[test]
    fn test_css_properties_serialization() {
        use crate::ui::div;
        use std::cell::RefCell;
        use std::rc::Rc;

        let engine = Rc::new(RefCell::new(FlexEngine::new()));

        let node_id = {
            let mut e = engine.borrow_mut();
            let id = e.add_node(100.0);
            // Set imperative field directly to test merging
            e.cpu_nodes[id as usize].max_width = 10.0;
            id
        };

        engine.borrow_mut().render();

        unsafe {
            let engine_borrow = engine.borrow();
            let nodes = engine_borrow.get_nodes_ptr();
            let gpu_node = &*nodes.add(node_id as usize);

            // Should be copied in flatten()
            assert_eq!(gpu_node.max_width, 10.0);

            // Verify that the style buffer grew (meaning styles were serialized)
            assert!(engine_borrow.get_node_class_list_and_inline_styles_count() > 0);

            // We can also verify that style resolution pass reset it in our simulated shader logic,
            // but for now, this confirms the CPU-side data flow is correct.
        }
    }
}
