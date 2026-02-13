<script>
    #[derive(Clone)]
    pub struct Props {
        pub text: String,
    }
</script>

<template>
    <div class="component-container">
        <text text="I am a component!" class="white-text" />
        <text :text="props.text" class="yellow-text" />
    </div>
</template>

<style>
.component-container {
    height: 100px;
    width: 200px;
    background-color: #884444;
    flex-direction: column;
}
.white-text {
    color: #ffffff;
}
.yellow-text {
    color: #ffff00;
}
</style>
