<script>
    #[derive(Clone)]
    pub struct Props {
        pub text: String,
    }
</script>

<template>
    <div style="height: 100px; width: 200px; background-color: #884444; flex-direction: column;">
        <text text="I am a component!" style="color: #ffffff;" />
        <text :text="props.text" style="color: #ffff00;" />
    </div>
</template>
