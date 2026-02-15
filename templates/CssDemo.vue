<template>
  <div class="demo-root">
    <div class="demo-header">
      <div class="demo-title">CSS Demo</div>
      <div class="demo-subtitle">Shorthand + layout + typography</div>
    </div>

    <div class="demo-grid">
      <div class="demo-card card-a">
        <div class="card-title">Borders + Outline</div>
        <div class="chip-row">
          <div class="chip chip-red">red</div>
          <div class="chip chip-green">green</div>
          <div class="chip chip-blue">blue</div>
        </div>
      </div>

      <div class="demo-card card-b">
        <div class="card-title">Spacing + Shadow</div>
        <div class="stack">
          <div class="stack-item">padding</div>
          <div class="stack-item">margin</div>
          <div class="stack-item">box-shadow</div>
        </div>
      </div>

      <div class="demo-card card-c">
        <div class="card-title">Position + Z Index</div>
        <div class="position-stage">
          <div class="square back">Back</div>
          <div class="square front">Front</div>
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
  flex-direction: row;
  justify-content: space-between;
  align-items: flex-start;
}

.demo-card {
  width: 32%;
  height: 240px;
  background-color: #1f1f1f;
  padding: 16px;
  margin-right: 12px;
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
  margin-bottom: 12px;
}

.card-a {
  border-width: 1px 2px 3px 4px;
  border-color: #ff7b7b #7bff9f #7bb7ff #ffe27b;
  outline-width: 1px;
  outline-color: cyan;
  outline-offset: 3px;
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
  border: 1px solid #3a3a3a;
  color: white;
  font-size: 12px;
}

.chip-red {
  color: red;
}

.chip-green {
  color: green;
}

.chip-blue {
  color: blue;
}

.card-b {
  border-top: 3px solid #f39c12;
  border-right: 2px solid #f1c40f;
  border-bottom: 3px solid #e67e22;
  border-left: 2px solid #d35400;
}

.stack {
  flex-direction: column;
  padding: 6px;
}

.stack-item {
  background-color: #2a2a2a;
  color: #e5e5e5;
  margin: 6px 0;
  padding: 10px 12px;
  border: 1px solid #404040;
}

.card-c {
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
</style>
