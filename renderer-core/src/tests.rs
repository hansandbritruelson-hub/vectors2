
#[cfg(test)]
mod tests {
    use crate::signals::{create_signal, create_effect};
    use crate::FlexEngine;
    use crate::ui::div;

    // We can't access FlexEngine easily here without initializing wasm-bindgen
    // So we just test the signals system itself first
    
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
        
        // Initial run
        assert_eq!(*result.borrow(), 0);
        
        // Update
        set_count.set(10);
        assert_eq!(*result.borrow(), 20);
        
        set_count.set(5);
        assert_eq!(*result.borrow(), 10);
    }

    #[test]
    fn test_builder_structure() {
        use crate::ui::{div, text};
        // Just verify the builder methods compile and run without panic
        // We can't call .build(engine) because Engine requires WASM environment mocked
        
        let _layout = div().width(100.0).row()
            .child(text("Hello"))
            .child(div().col());
            
        assert!(true);
    }

    #[test]
    fn test_reactive_flags() {
        // This test requires FlexEngine to be constructible in test environment
        // Since FlexEngine uses wasm_bindgen, this might fail if we don't mock the font loading
        // However, our `new()` implementation tries to load bytes.
        // Let's rely on the fact that `ttf_parser` works in standard Rust too.
        
        let mut engine = FlexEngine::new();
        let (visible, set_visible) = create_signal(1u32);
        
        let node_id = div()
            .bind_flags(visible)
            .build(&mut engine, None);
            
        // Initial State (1)
        // We need to check engine.nodes[node_id].flags
        // Access via raw pointer or unsafe access if fields are private?
        // Wait, fields are public in lib.rs
        
        unsafe {
            let nodes = engine.get_nodes_ptr();
            let flags = (*nodes.add(node_id as usize)).flags;
            assert_eq!(flags, 1);
        }
        
        // Update to 0 (Hidden)
        set_visible.set(0);
        
        unsafe {
            let nodes = engine.get_nodes_ptr();
            let flags = (*nodes.add(node_id as usize)).flags;
            assert_eq!(flags, 0);
        }
        
        // Update to 1 (Visible)
        set_visible.set(1);
        
        unsafe {
            let nodes = engine.get_nodes_ptr();
            let flags = (*nodes.add(node_id as usize)).flags;
            assert_eq!(flags, 1);
        }
    }
}
