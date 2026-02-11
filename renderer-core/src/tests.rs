
#[cfg(test)]
mod tests {
    use crate::signals::{create_signal, create_effect};
    use crate::FlexEngine;
    use crate::ui::div;

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
        let _layout = div().width(100.0).row()
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
        
        let node_id = div()
            .bind_flags(visible)
            .build(engine.clone(), None);
            
        engine.borrow_mut().render();

        unsafe {
            let engine_borrow = engine.borrow();
            let nodes = engine_borrow.get_nodes_ptr();
            let flags = (*nodes.add(node_id as usize)).flags;
            assert_eq!(flags, 1);
        }
        
        set_visible.set(0);
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
        use std::cell::RefCell;
        use std::rc::Rc;
        use crate::ui::mount_list;

        let engine = Rc::new(RefCell::new(FlexEngine::new()));
        let (items, set_items) = create_signal(vec!["A".to_string(), "B".to_string()]);
        
        let root_id = {
            let mut e = engine.borrow_mut();
            e.add_node(100.0)
        };

        mount_list(engine.clone(), root_id, items, |name| name.clone(), |name| {
            div().text(&name)
        });

        engine.borrow_mut().render();
        assert_eq!(engine.borrow().get_node_count(), 4);

        set_items.set(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
        engine.borrow_mut().render();
        assert_eq!(engine.borrow().get_node_count(), 5);

        set_items.set(vec!["X".to_string()]);
        engine.borrow_mut().render();
        assert_eq!(engine.borrow().get_node_count(), 3);
    }
}
