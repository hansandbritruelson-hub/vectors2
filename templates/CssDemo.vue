<template>
  <div class="demo-root">
    <div class="demo-header">
      <div class="demo-title">CSS Feature Demo</div>
      <div class="demo-subtitle">Showcasing all currently supported style groups</div>
    </div>

    <div class="demo-grid">
      <div class="demo-row">
        <div class="demo-card card-a">
          <div class="card-title">Layout + Sizing</div>
          <div class="layout-stage">
            <div class="layout-item one">100 x 44</div>
            <div class="layout-item two">130 x 36</div>
            <div class="layout-item three">88 x 52</div>
          </div>
          <div class="feature-note">`width` `height` `flex-direction` `justify-content` `align-items`</div>
        </div>

        <div class="demo-card card-b">
          <div class="card-title">Spacing + Shadow</div>
          <div class="spacing-stage">
            <div class="margin-shell">
              <div class="padding-shell">
                <div class="content-chip">content</div>
              </div>
            </div>
            <div class="shadow-chip">shadow</div>
          </div>
          <div class="feature-note">`margin` `padding` `box-shadow`</div>
        </div>

        <div class="demo-card card-c">
          <div class="card-title">Borders + Outline</div>
          <div class="chip-row">
            <div class="chip chip-red">top</div>
            <div class="chip chip-green">right</div>
            <div class="chip chip-blue">left</div>
          </div>
          <div class="feature-note">`border` `border-width` `border-color` `outline` `outline-offset`</div>
        </div>
      </div>

      <div class="demo-row">
        <div class="demo-card card-d">
          <div class="card-title">Position + Z Index</div>
          <div class="position-stage">
            <div class="square back">Back</div>
            <div class="square front">Front</div>
          </div>
          <div class="feature-note">`position` `top` `left` `z-index`</div>
        </div>

        <div class="demo-card card-e">
          <div class="card-title">Typography</div>
          <div class="type-large">Typography Demo</div>
          <div class="type-row type-center">center align + wider spacing</div>
          <div class="type-row type-right">italic right aligned</div>
          <div class="type-transform">mixed case becomes uppercase</div>
          <div class="feature-note">`font-size` `font-weight` `font-style` `line-height` `letter-spacing` `word-spacing` `text-align` `text-transform`</div>
        </div>

        <div class="demo-card card-f">
          <div class="card-title">Color + Paint</div>
          <div class="paint-text">Text color and background color</div>
          <div class="paint-vector">Fill Stroke</div>
          <div class="feature-note">`background-color` `color` `fill` `stroke` `stroke-width`</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
.demo-root {
  width: 100%;
  height: 100%;
  flex-direction: column;
  background-color: #141414;
  padding: 20px;
}

.demo-header {
  flex-direction: column;
  margin-bottom: 14px;
}

.demo-title {
  font-size: 30px;
  color: #ffffff;
}

.demo-subtitle {
  font-size: 14px;
  color: #9e9e9e;
  margin-top: 4px;
}

.demo-grid {
  flex-direction: column;
}

.demo-row {
  flex-direction: row;
  justify-content: space-between;
  margin-bottom: 12px;
}

.demo-row:last-child {
  margin-bottom: 0;
}

.demo-card {
  width: 32%;
  height: 250px;
  background-color: #1f1f1f;
  padding: 16px;
  margin-right: 1.5%;
  border: 1px solid #2d2d2d;
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.35);
  flex-direction: column;
}

.demo-card:last-child {
  margin-right: 0;
}

.card-title {
  font-size: 16px;
  color: #f2f2f2;
  margin-bottom: 10px;
}

.card-a {
  background-color: #1c222e;
  border-color: #2d425c;
}

.layout-stage {
  height: 140px;
  background-color: #111923;
  flex-direction: row;
  align-items: center;
  justify-content: space-evenly;
  padding: 8px;
}

.layout-item {
  justify-content: center;
  align-items: center;
  color: #dce9ff;
  font-size: 11px;
  border: 1px solid #5578a4;
  background-color: #2b3d55;
}

.layout-item.one {
  width: 100px;
  height: 44px;
}

.layout-item.two {
  width: 130px;
  height: 36px;
}

.layout-item.three {
  width: 88px;
  height: 52px;
}

.chip-row {
  flex-direction: row;
  align-items: center;
  justify-content: space-evenly;
  margin-top: 8px;
}

.chip {
  background-color: #2b2b2b;
  padding: 8px 10px;
  border-width: 2px 1px 3px 4px;
  color: white;
  font-size: 12px;
}

.chip-red {
  border-color: #e74c3c #bf3f32 #a6372b #c74435;
  outline-width: 1px;
  outline-color: #ff9a9a;
  outline-offset: 2px;
}

.chip-green {
  border-color: #2ecc71 #28b966 #249f59 #2ab866;
  outline-width: 2px;
  outline-color: #8fffbe;
  outline-offset: 1px;
}

.chip-blue {
  border-color: #3498db #2f89c4 #2a78ad #2f89c4;
  outline-width: 1px;
  outline-color: #8ecfff;
  outline-offset: 3px;
}

.card-b {
  background-color: #24201a;
  border-color: #5a4421;
}

.spacing-stage {
  height: 140px;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-color: #19140e;
}

.margin-shell {
  background-color: #6b3b12;
  margin: 8px 18px 10px 18px;
  padding: 10px;
}

.padding-shell {
  background-color: #b56a27;
  padding: 14px 22px 8px 12px;
}

.content-chip {
  background-color: #f7d8b2;
  color: #2a1b0d;
  padding: 4px 8px;
}

.shadow-chip {
  margin-top: 10px;
  background-color: #2b2b2b;
  color: #f2f2f2;
  padding: 8px 12px;
  box-shadow: 10px 6px 14px 3px rgba(0, 0, 0, 0.45);
}

.card-c {
  border-width: 1px 2px 3px 4px;
  border-color: #ff7b7b #7bff9f #7bb7ff #ffe27b;
  outline-width: 1px;
  outline-color: cyan;
  outline-offset: 3px;
}

.card-d {
  border: 1px solid #2f2f2f;
}

.position-stage {
  position: relative;
  height: 160px;
  background-color: #171717;
  border: 1px solid #2b2b2b;
  margin-top: 8px;
}

.square {
  position: absolute;
  width: 100px;
  height: 70px;
  justify-content: center;
  align-items: center;
  color: #ffffff;
  font-size: 13px;
}

.back {
  top: 44px;
  left: 32px;
  z-index: 1;
  background-color: #2980b9;
}

.front {
  top: 70px;
  left: 70px;
  z-index: 5;
  background-color: #e74c3c;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.4);
}

.card-e {
  background-color: #1f1d27;
  border-color: #4a4165;
  color: #f4f2ff;
}

.type-large {
  font-size: 26px;
  font-weight: 700;
  line-height: 1.1;
}

.type-row {
  width: 100%;
  margin-top: 6px;
  padding: 4px 6px;
  background-color: #2e2940;
}

.type-center {
  text-align: center;
  letter-spacing: 0.1em;
  word-spacing: 0.25em;
}

.type-right {
  text-align: right;
  font-style: italic;
  font-weight: 300;
}

.type-transform {
  margin-top: 6px;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: #cfc7ff;
}

.card-f {
  background-color: #1e2321;
  border-color: #3e5e52;
}

.paint-text {
  background-color: #164336;
  color: #8fffd0;
  padding: 8px 10px;
}

.paint-vector {
  margin-top: 10px;
  padding: 6px 8px;
  fill: #f1c40f;
  stroke: #111111;
  stroke-width: 1px;
  font-weight: 700;
}

.feature-note {
  margin-top: 10px;
  color: #b3b3b3;
  font-size: 10px;
  line-height: 1.3;
}
</style>
