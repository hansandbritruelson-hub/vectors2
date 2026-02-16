<template>
    <div class="chat-container">
        <div class="chat-header" @click="(toggle_open)(())">
            <div class="chat-title">Assistant</div>
            <div class="chat-toggle">{{ open ? "v" : "^" }}</div>
        </div>
        <div class="chat-body" v-if="open">
            <div class="messages-list">
                <div class="message system">
                    Hello! How can I help you with your design today?
                </div>
                <div v-for="msg in messages" class="message user">
                    {{ msg.text }}
                </div>
            </div>
            <div class="chat-input-area">
                <input 
                    class="chat-input"
                    type="text"
                    :value="input_text"
                    @update:modelValue="move |val| set_input_text.set(val)"
                />
                <div class="send-button" @click="(send_message.get())(())">Send</div>
            </div>
        </div>
    </div>
</template>

<script>
    // Codegen provides create_signal, create_effect, etc.
    
    #[derive(Clone)]
    pub struct Message {
        pub id: String,
        pub text: String,
        pub sender: String,
    }

    pub struct Props { }

    let (open, set_open) = create_signal(true);
    let (input_text, set_input_text) = create_signal("".to_string());
    let (messages, set_messages) = create_signal::<Vec<Message>>(vec![]);

    let toggle_open = Rc::new(move |_| {
        set_open.set(!open.get());
    });

    let send_message_logic = Rc::new(move |_| {
        let text = input_text.get();
        if text.len() > 0 {
            let mut current = messages.get();
            current.push(Message {
                id: format!("{}", current.len()),
                text: text,
                sender: "user".to_string(),
            });
            set_messages.set(current);
            set_input_text.set("".to_string());
        }
    });
    
    // Wrap in signal to make it Copy-able for v-if closures
    let (send_message, _) = create_signal(send_message_logic);
</script>

<style>
.chat-container {
    position: absolute;
    right: 20px;
    bottom: 0px;
    width: 320px;
    background-color: #1e1e1e;
    border-top-left-radius: 12px;
    border-top-right-radius: 12px;
    box-shadow: 0 4px 20px rgba(0,0,0,0.5);
    flex-direction: column;
    z-index: 1000;
}

.chat-header {
    height: 48px;
    background-color: #3498db;
    border-top-left-radius: 12px;
    border-top-right-radius: 12px;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    cursor: pointer;
}

.chat-title {
    color: #ffffff;
    font-weight: 700;
    font-size: 16px;
}

.chat-toggle {
    color: #ffffff;
    font-weight: 700;
}

.chat-body {
    height: 400px;
    flex-direction: column;
    background-color: #252526;
}

.messages-list {
    flex: 1;
    padding: 16px;
    flex-direction: column;
    overflow-y: auto;
}

.message {
    padding: 10px 14px;
    background-color: #333333;
    color: #e0e0e0;
    border-radius: 8px;
    margin-bottom: 10px;
    font-size: 14px;
    line-height: 1.4;
}

.message.system {
    background-color: #3e3e42;
    border-left-width: 4px;
    border-color-left: #3498db;
}

.message.user {
    background-color: #3498db;
    color: #ffffff;
    align-self: flex-end;
}

.chat-input-area {
    height: 60px;
    border-top-width: 1px;
    border-color-top: #333;
    padding: 10px;
    flex-direction: row;
    background-color: #1e1e1e;
    align-items: center;
}

.chat-input {
    flex: 1;
    height: 40px;
    background-color: #2d2d2d;
    border-radius: 20px;
    color: #ffffff;
    padding: 0 16px;
    font-size: 14px;
    border: 1px solid #444;
}

.send-button {
    margin-left: 10px;
    height: 40px;
    padding: 0 16px;
    background-color: #3498db;
    border-radius: 20px;
    justify-content: center;
    align-items: center;
    color: #ffffff;
    font-weight: 700;
    font-size: 14px;
    cursor: pointer;
}

.send-button:hover {
    background-color: #2980b9;
}
</style>
