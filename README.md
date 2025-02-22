# Terp

Interpolated drawing app, using [Bevy](https://bevyengine.org/)


https://github.com/user-attachments/assets/d92ff8f7-669b-4b6f-b296-8fbe804c5b88


## Controls

Drag in left (source) or right (target) viewport to draw.
Corresponding source/target drawings will be paired and interpolated.
* `Size` button - click and drag to resize
* `Color` button - click and drag to change color
* `Undo` button - press to undo last drawing
* `Spacebar` to toggle interpolation
* `S` to save project (if run with `terp editor --project .../path/to/project.terp`)

## Demos

Demos require a web browser that supports [WebGPU](https://caniuse.com/webgpu).

* Terp [Player](https://rectalogic.com/terp/player.html)
* Terp [Editor](https://rectalogic.com/terp/editor.html)

## Releases

You can download pre-built releases [here](https://github.com/rectalogic/terp/releases).
For macOS you will need to `xattr -dr com.apple.quarantine terp`

---

Inspired by [Lizard Ladder](http://www.tedwiggin.com/LizardLadder/)
