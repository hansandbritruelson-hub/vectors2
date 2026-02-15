<template>
  <div class="video-container">
    <div class="workspace">
      <div class="preview-area">
        <div class="player-container">
          <div class="video-mock">
            <div class="play-overlay">
              <img src="asset://phosphor/play.svg" class="icon-play-large" />
            </div>
          </div>
          <div class="player-controls">
            <div class="control-btn">
              <img src="asset://phosphor/skip-back.svg" class="icon-control" />
            </div>
            <div class="control-btn active">
              <img src="asset://phosphor/play.svg" class="icon-control" />
            </div>
            <div class="control-btn">
              <img src="asset://phosphor/skip-forward.svg" class="icon-control" />
            </div>
            <div class="time-display">00:04:12 / 00:15:00</div>
            <div class="spacer"></div>
            <div class="control-btn">
              <img src="asset://phosphor/settings.svg" class="icon-control" />
            </div>
          </div>
        </div>
      </div>

      <div class="timeline-area">
        <div class="timeline-header">
          <div class="timeline-tool">
            <img src="asset://phosphor/selection.svg" class="icon-mini" />
          </div>
          <div class="timeline-tool">
            <img src="asset://phosphor/cut.svg" class="icon-mini" />
          </div>
          <div class="spacer"></div>
          <div class="zoom-controls">
             <img src="asset://phosphor/zoom-in.svg" class="icon-mini" />
          </div>
        </div>
        <div class="timeline-ruler">
          <div class="ruler-mark">0s</div>
          <div class="ruler-mark">5s</div>
          <div class="ruler-mark">10s</div>
          <div class="ruler-mark">15s</div>
        </div>
        <div class="tracks">
          <div class="track">
            <div class="track-label">Video 1</div>
            <div class="track-content">
              <div class="clip orange" style="width: 200px; left: 50px;">Clip_01.mp4</div>
              <div class="clip orange" style="width: 150px; left: 300px;">Clip_02.mp4</div>
            </div>
          </div>
          <div class="track">
            <div class="track-label">Audio 1</div>
            <div class="track-content">
              <div class="clip blue" style="width: 400px; left: 50px;">Background_Music.wav</div>
            </div>
          </div>
        </div>
        <div class="playhead"></div>
      </div>
    </div>
  </div>
</template>

<style>
.video-container {
  width: 100%;
  height: 100%;
  background-color: #121212;
  flex-direction: column;
}

.workspace {
    flex-direction: column;
}

/* Preview Area */
.preview-area {
    background-color: #0c0c0c;
  justify-content: center;
  align-items: center;
  /* GPU shader unsupported for now: padding: 24px; */
}

.player-container {
  width: 80%;
  /* GPU shader unsupported for now: aspect-ratio: 1.777; */ /* 16:9 */
  background-color: #1a1a1a;
  /**border-radius: 12px;**/  /**overflow: hidden;**/  /* GPU shader unsupported for now: box-shadow: 0 20px 50px rgba(0,0,0,0.6); */
  flex-direction: column;
}

.video-mock {
    background-color: #000;
  justify-content: center;
  align-items: center;
  position: relative;
}

.play-overlay {
  width: 80px;
  height: 80px;
  /**border-radius: 40px;**/  background-color: rgba(255,255,255,0.1);
  justify-content: center;
  align-items: center;
  /* GPU shader unsupported for now: border: 2px solid rgba(255,255,255,0.2); */
}

.icon-play-large {
  width: 40px;
  height: 40px;
  /* GPU shader unsupported for now: opacity: 0.8; */
}

.player-controls {
  height: 60px;
  background-color: #1e1e1e;
  flex-direction: row;
  align-items: center;
  /* GPU shader unsupported for now: padding: 0 20px; */
  /* GPU shader unsupported for now: border-top: 1px solid #333; */
}

.control-btn {
  width: 40px;
  height: 40px;
  justify-content: center;
  align-items: center;
  /**border-radius: 20px;**/  /* GPU shader unsupported for now: margin: 0 4px; */
}

.control-btn:hover {
  background-color: #333;
}

.control-btn.active {
  color: #3498db;
}

.icon-control {
  width: 20px;
  height: 20px;
}

.time-display {
  margin-left: 20px;
  font-size: 14px;
  color: #888;
  
}

/* Timeline Area */
.timeline-area {
  height: 300px;
  background-color: #1e1e1e;
  /* GPU shader unsupported for now: border-top: 1px solid #333; */
  flex-direction: column;
  position: relative;
}

.timeline-header {
  height: 40px;
  background-color: #252525;
    flex-direction: row;
  align-items: center;
  /* GPU shader unsupported for now: padding: 0 12px; */
}

.timeline-tool {
  width: 28px;
  height: 28px;
  justify-content: center;
  align-items: center;
  /**border-radius: 4px;**/  margin-right: 4px;
}

.timeline-tool:hover {
  background-color: #383838;
}

.timeline-ruler {
  height: 24px;
  background-color: #1a1a1a;
    flex-direction: row;
  /* GPU shader unsupported for now: padding-left: 100px; */
}

.ruler-mark {
  width: 100px;
  font-size: 10px;
  color: #555;
  /* GPU shader unsupported for now: padding-top: 4px; */
  /* GPU shader unsupported for now: border-left: 1px solid #222; */
  /* GPU shader unsupported for now: padding-left: 4px; */
}

.tracks {
    flex-direction: column;
  /**overflow-y: scroll;**/}

.track {
  height: 60px;
  flex-direction: row;
  }

.track-label {
  width: 100px;
  background-color: #222;
  justify-content: center;
  /* GPU shader unsupported for now: padding-left: 12px; */
  font-size: 12px;
  color: #888;
  /* GPU shader unsupported for now: border-right: 1px solid #333; */
}

.track-content {
    position: relative;
  background-color: #161616;
}

.clip {
  position: absolute;
  top: 10px;
  //bottom: 10px;
  /**border-radius: 4px;**/  /* GPU shader unsupported for now: padding: 8px; */
  font-size: 11px;
  color: rgba(255,255,255,0.8);
  /* GPU shader unsupported for now: border: 1px solid rgba(0,0,0,0.2); */
}

.clip.orange {
  background-color: #d35400;
}

.clip.blue {
  background-color: #2980b9;
}

.playhead {
  position: absolute;
  top: 40px;
  //bottom: 0;
  left: 350px;
  width: 2px;
  background-color: #e74c3c;
  z-index: 10;
}

.playhead::after {
  //content: "";
  position: absolute;
  top: 0;
  left: -4px;
  width: 0;
  height: 0;
  /* GPU shader unsupported for now: border-left: 5px solid transparent; */
  /* GPU shader unsupported for now: border-right: 5px solid transparent; */
  /* GPU shader unsupported for now: border-top: 8px solid #e74c3c; */
}

.icon-mini {
  width: 16px;
  height: 16px;
}

.spacer {
  }
</style>
