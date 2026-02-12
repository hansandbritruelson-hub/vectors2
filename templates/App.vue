<template>
  <div class="main">
    <div class="sidebar" @click="set_count.set(count.get() + 1)">
      <div class="count-text">Count: {{ count }}</div>
      <div :text="sidebar_content"></div>
      <div class="icon" image="paintbrush.svg"></div>
      <div class="icon" image="paintbrush.svg"></div>
    </div>
    
    <div class="right-pane">
      <div class="row1">
        <div class="r1-left">Row 1aa - Left Div</div>
        <div class="r1-right">Row 1 - Right Div</div>
      </div>
      
      <div class="curve-test" style="height: 150px; background-color: #222;">
        <bezier-curve d="M 10 10 L 90 10 L 90 90 Z" style="width: 100px; height: 100px; color: rgba(255, 0, 0, 1.0);" />
        <bezier-curve d="M 10 10 C 10 10, 50 10, 50 50 C 50 90, 90 90, 90 90" style="width: 100px; height: 100px; color: rgba(0, 0, 255, 1.0);" />
      </div>
      
      <div class="row2">
        <div class="r2-text1">This is a reasonably long piece of text that is intended to test the wrapping capabilities of our flex engine. It should flow nicely within its container.</div>
        <div class="r2-text2">Another long block of text here, serving as the second part of Row 2. We want to ensure that multiple wrapping blocks can coexist side-by-side in a row.</div>
      </div>
      
      <div class="row3">
        <div>Row 3: Keyed Reusable List (v4 Sample):</div>
        <div v-for="user in users" class="user-item">
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
.main {
    flex-direction: row;
    background-color: #1a1a1a;
}
.sidebar {
    flex-direction: column;
    width: 75px;
    background-color: #333340;
}
.count-text {
    color: #ff00ff;
}
.icon {
    width: 64px;
    height: 64px;
}
.right-pane {
    flex-direction: column;
    background-color: #262626;
}
.row1 {
    flex-direction: row;
    background-color: #333333;
}
.r1-left { background-color: #4d4d59; }
.r1-right { background-color: #594d4d; }

.row2 {
    flex-direction: row;
    background-color: #334033;
}
.r2-text1 { background-color: #405940; }
.r2-text2 { background-color: #404059; }

.row3 {
    flex-direction: column;
    background-color: #1a1a1a;
}
.user-item {
    background-color: #4db34d;
}
</style>
