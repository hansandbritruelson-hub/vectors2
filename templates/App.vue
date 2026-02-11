<template>
  <div class="main" row color="0.1, 0.1, 0.1, 1.0">
    <div class="sidebar" col width="75" color="0.2, 0.2, 0.25, 1.0" @click="set_count.set(count.get() + 1)">
      <div color="1,0,1,1">Count: {{ count }}</div>
      <div :text="sidebar_content"></div>
      <div width="64" height="64" image="paintbrush.svg"></div>
      <div width="64" height="64" image="paintbrush.svg"></div>
    </div>
    
    <div class="right-pane" col color="0.15, 0.15, 0.15, 1.0">
      <div class="row1" row color="0.2, 0.2, 0.2, 1.0">
        <div color="0.3, 0.3, 0.35, 1.0">Row 1aa - Left Div</div>
        <div color="0.35, 0.3, 0.3, 1.0">Row 1 - Right Div</div>
      </div>
      
      <div class="row2" row color="0.2, 0.25, 0.2, 1.0">
        <div color="0.25, 0.35, 0.25, 1.0">This is a reasonably long piece of text that is intended to test the wrapping capabilities of our flex engine. It should flow nicely within its container.</div>
        <div color="0.25, 0.25, 0.35, 1.0">Another long block of text here, serving as the second part of Row 2. We want to ensure that multiple wrapping blocks can coexist side-by-side in a row.</div>
      </div>
      
      <div class="row3" col color="0.1, 0.1, 0.1, 1.0">
        <div>Row 3: Keyed Reusable List (v4 Sample):</div>
        <div v-for="user in users" color="0.3, 0.7, 0.3, 1.0">
          {{ user.name }}
        </div>
      </div>
    </div>
  </div>
</template>

<script>
    let (sidebar_content, set_sidebar_content) = crate::signals::create_signal("SIDEBAR\n(Reactive)".to_string());
    let (count, set_count) = crate::signals::create_signal(0);
    
    #[derive(Clone)]
    struct User { id: String, name: String }
    
    let (users, set_users) = crate::signals::create_signal(vec![
        User { id: "1".into(), name: "Alice".into() },
        User { id: "2".into(), name: "Bob".into() },
    ]);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        
        let mut count = 0;
        let closure = Closure::wrap(Box::new(move || {
            count += 1;
            set_sidebar_content.set(format!("SIDEBAR\nTick: {}", count));
            
            if count % 2 == 0 {
                set_users.set(vec![
                    User { id: "1".into(), name: "Alice".into() },
                    User { id: "3".into(), name: format!("New User {}", count) },
                    User { id: "2".into(), name: "Bob (Moved)".into() },
                ]);
            } else {
                set_users.set(vec![
                    User { id: "2".into(), name: "Bob".into() },
                    User { id: "1".into(), name: "Alice".into() },
                ]);
            }
        }) as Box<dyn FnMut()>);
        
        if let Some(window) = crate::web_bindings::get_window() {
            window.set_interval(closure.as_ref().unchecked_ref(), 2000);
        }
        closure.forget();
    }
</script>

<style>
.root {
    display: flex;
}
</style>
